use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    errors::ContractError,
    events::*,
    state::{bonding_curve::*, global::*},
};

use crate::state::bonding_curve::locker::{BondingCurveLockerCtx, IntoBondingCurveLockerCtx};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapParams {
    pub base_in: bool,
    pub exact_in_amount: u64,
    pub min_out_amount: u64,
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(params: SwapParams)]
pub struct Swap<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        seeds = [Global::SEED_PREFIX.as_bytes()],
        constraint = global.initialized == true @ ContractError::NotInitialized,
        bump,
    )]
    global: Box<Account<'info, Global>>,

    #[account(mut)]
    /// CHECK: fee receiver asserted in validation function
    fee_receiver: AccountInfo<'info>,

    mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [BondingCurve::SEED_PREFIX.as_bytes(), mint.to_account_info().key.as_ref()],
        constraint = bonding_curve.complete == false @ ContractError::BondingCurveComplete,
        bump,
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    bonding_curve_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [BondingCurve::SOL_ESCROW_SEED_PREFIX.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    /// CHECK: PDA to hold SOL for bonding curve
    pub bonding_curve_sol_escrow: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    user_token_account: Box<Account<'info, TokenAccount>>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    associated_token_program: Program<'info, AssociatedToken>,

    clock: Sysvar<'info, Clock>,
}
impl<'info> IntoBondingCurveLockerCtx<'info> for Swap<'info> {
    fn into_bonding_curve_locker_ctx(
        &self,
        bonding_curve_bump: u8,
    ) -> BondingCurveLockerCtx<'info> {
        BondingCurveLockerCtx {
            bonding_curve_bump,
            mint: self.mint.clone(),
            global: self.global.clone(),
            bonding_curve: self.bonding_curve.clone(),
            bonding_curve_token_account: self.bonding_curve_token_account.clone(),
            bonding_curve_sol_escrow: self.bonding_curve_sol_escrow.clone(),
            token_program: self.token_program.clone(),
        }
    }
}
impl Swap<'_> {
    pub fn validate(&self, params: &SwapParams) -> Result<()> {
        let SwapParams {
            base_in: _,
            exact_in_amount,
            min_out_amount: _,
        } = params;
        let clock = Clock::get()?;

        let is_dev_buy = clock.slot == self.bonding_curve.start_slot
            && self.user.key() == self.bonding_curve.creator;

        if !is_dev_buy {
            require!(
                self.bonding_curve.is_started(&clock),
                ContractError::CurveNotStarted
            );
        }

        require!(exact_in_amount > &0, ContractError::MinSwap);

        require!(
            self.fee_receiver.key() == self.global.fee_receiver,
            ContractError::InvalidFeeReceiver
        );

        require!(
            !self.global.is_config_outdated()?,
            ContractError::ConfigOutdated
        );

        Ok(())
    }
    pub fn handler(ctx: Context<Swap>, params: SwapParams) -> Result<()> {
        let SwapParams {
            base_in,
            exact_in_amount,
            min_out_amount,
        } = params;

        msg!(
            "Swap started. BaseIn: {}, AmountIn: {}, MinOutAmount: {}",
            base_in,
            exact_in_amount,
            min_out_amount
        );
        let bonding_curve = ctx.accounts.bonding_curve.clone();
        let locker: &mut BondingCurveLockerCtx = &mut ctx
            .accounts
            .into_bonding_curve_locker_ctx(ctx.bumps.bonding_curve);
        locker.unlock_ata()?;

        let sol_amount: u64;
        let token_amount: u64;
        let fee_lamports: u64;

        if base_in {
            // Sell tokens
            require!(
                ctx.accounts.user_token_account.amount >= exact_in_amount,
                ContractError::InsufficientUserTokens,
            );

            let sell_result = ctx
                .accounts
                .bonding_curve
                .apply_sell(exact_in_amount)
                .ok_or(ContractError::SellFailed)?;

            msg!("SellResult: {:#?}", sell_result);

            sol_amount = sell_result.sol_amount;
            token_amount = sell_result.token_amount;

            let clock = Clock::get()?;
            fee_lamports = bonding_curve.calculate_fee(sol_amount, clock.slot)?;
            msg!("Fee: {} SOL", fee_lamports);

            Swap::complete_sell(&ctx, sell_result.clone(), min_out_amount, fee_lamports)?;
        } else {
            // Buy tokens
            let rent = Rent::get()?;
            let min_rent = rent.minimum_balance(0); // 0 for data size since this is just a native SOL account
            require!(
                ctx.accounts.user.get_lamports() >= exact_in_amount.checked_add(min_rent).unwrap(),
                ContractError::InsufficientUserSOL,
            );

            let clock = Clock::get()?;
            let buy_amount_applied: u64;

            // Check if slot is start slot and buyer is bonding_curve creator
            if clock.slot == bonding_curve.start_slot
                && ctx.accounts.user.key() == bonding_curve.creator
            {
                msg!("Dev buy");
                fee_lamports = 0;
                buy_amount_applied = exact_in_amount;
            } else {
                fee_lamports = bonding_curve.calculate_fee(exact_in_amount, clock.slot)?;
                msg!("Fee: {} SOL", fee_lamports);
                buy_amount_applied = exact_in_amount - fee_lamports;
            }

            let buy_result = ctx
                .accounts
                .bonding_curve
                .apply_buy(buy_amount_applied)
                .ok_or(ContractError::BuyFailed)?;

            msg!("BuyResult: {:#?}", buy_result);

            sol_amount = buy_result.sol_amount;
            token_amount = buy_result.token_amount;

            Swap::complete_buy(&ctx, buy_result.clone(), min_out_amount, fee_lamports)?;
        }

        BondingCurve::invariant(
            &mut ctx
                .accounts
                .into_bonding_curve_locker_ctx(ctx.bumps.bonding_curve),
        )?;
        let bonding_curve = &ctx.accounts.bonding_curve;

        // Emit trade event used for indexing
        emit_cpi!(TradeEvent {
            mint: *ctx.accounts.mint.to_account_info().key,
            sol_amount: sol_amount,
            token_amount: token_amount,
            fee_lamports: fee_lamports,
            is_buy: !base_in,
            user: *ctx.accounts.user.to_account_info().key,
            timestamp: Clock::get()?.unix_timestamp,
            virtual_sol_reserves: bonding_curve.virtual_sol_reserves,
            virtual_token_reserves: bonding_curve.virtual_token_reserves,
            real_sol_reserves: bonding_curve.real_sol_reserves,
            real_token_reserves: bonding_curve.real_token_reserves,
        });

        // Emit complete event when bonding curve is completed
        if bonding_curve.complete {
            emit_cpi!(CompleteEvent {
                user: *ctx.accounts.user.to_account_info().key,
                mint: *ctx.accounts.mint.to_account_info().key,
                virtual_sol_reserves: bonding_curve.virtual_sol_reserves,
                virtual_token_reserves: bonding_curve.virtual_token_reserves,
                real_sol_reserves: bonding_curve.real_sol_reserves,
                real_token_reserves: bonding_curve.real_token_reserves,
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        msg!("{:#?}", bonding_curve);

        Ok(())
    }

    pub fn complete_buy(
        ctx: &Context<Swap>,
        buy_result: BuyResult,
        min_out_amount: u64,
        fee_lamports: u64,
    ) -> Result<()> {
        let bonding_curve = &ctx.accounts.bonding_curve;

        require!(
            buy_result.token_amount >= min_out_amount,
            ContractError::SlippageExceeded,
        );

        // Transfer tokens to user
        let cpi_accounts = Transfer {
            from: ctx.accounts.bonding_curve_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: bonding_curve.to_account_info(),
        };

        let signer = BondingCurve::get_signer(
            &ctx.bumps.bonding_curve,
            ctx.accounts.mint.to_account_info().key,
        );
        let signer_seeds = &[&signer[..]];
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            ),
            buy_result.token_amount,
        )?;
        let locker = &mut ctx
            .accounts
            .into_bonding_curve_locker_ctx(ctx.bumps.bonding_curve);
        locker.lock_ata()?;
        msg!("Token transfer complete");

        // Transfer SOL to bonding curve
        let transfer_instruction = system_instruction::transfer(
            ctx.accounts.user.key,
            ctx.accounts.bonding_curve_sol_escrow.to_account_info().key,
            buy_result.sol_amount,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.bonding_curve_sol_escrow.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        msg!("SOL to bonding curve transfer complete");

        // Transfer SOL to fee recipient
        let fee_transfer_instruction = system_instruction::transfer(
            ctx.accounts.user.key,
            &ctx.accounts.fee_receiver.key(),
            fee_lamports,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &fee_transfer_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.fee_receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        msg!("Fee transfer to fee_receiver complete");

        Ok(())
    }

    pub fn complete_sell(
        ctx: &Context<Swap>,
        sell_result: SellResult,
        min_out_amount: u64,
        fee_lamports: u64,
    ) -> Result<()> {
        // Sell tokens
        let sell_amount_minus_fee = sell_result.sol_amount - fee_lamports;

        require!(
            sell_amount_minus_fee >= min_out_amount,
            ContractError::SlippageExceeded,
        );

        // Transfer tokens to bonding curve
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.bonding_curve_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            sell_result.token_amount,
        )?;
        let locker = &mut ctx
            .accounts
            .into_bonding_curve_locker_ctx(ctx.bumps.bonding_curve);
        locker.lock_ata()?;

        msg!("Token to bonding curve transfer complete");

        // Transfer SOL to user
        let sol_escrow_signer = BondingCurve::get_sol_escrow_signer(
            &ctx.bumps.bonding_curve_sol_escrow,
            &ctx.accounts.mint.to_account_info().key,
        );
        let sol_escrow_signer_seeds = &[&sol_escrow_signer[..]];

        let sol_ix = system_instruction::transfer(
            &ctx.accounts.bonding_curve_sol_escrow.to_account_info().key,
            &ctx.accounts.user.to_account_info().key,
            sell_amount_minus_fee,
        );

        invoke_signed(
            &sol_ix,
            &[
                ctx.accounts
                    .bonding_curve_sol_escrow
                    .to_account_info()
                    .clone(),
                ctx.accounts.user.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            sol_escrow_signer_seeds,
        )?;
        msg!("SOL to user transfer complete");

        if fee_lamports > 0 {
            let fee_transfer_instruction = system_instruction::transfer(
                &ctx.accounts.bonding_curve_sol_escrow.to_account_info().key,
                &ctx.accounts.fee_receiver.to_account_info().key,
                fee_lamports,
            );

            invoke_signed(
                &fee_transfer_instruction,
                &[
                    ctx.accounts.bonding_curve_sol_escrow.to_account_info(),
                    ctx.accounts.fee_receiver.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                sol_escrow_signer_seeds,
            )?;

            msg!("Fee to fee_vault transfer complete");
        }
        Ok(())
    }
}
