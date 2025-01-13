use crate::constants::{METEORA_PROGRAM_KEY, QUOTE_MINT, TOKEN_VAULT_SEED};
use crate::state::bonding_curve::locker::{BondingCurveLockerCtx, IntoBondingCurveLockerCtx};
use crate::state::{bonding_curve::*, meteora::get_pool_create_ix_data};
use crate::{errors::ContractError, state::global::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::{instruction::Instruction, system_instruction};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use std::str::FromStr;

#[derive(Accounts)]
pub struct InitializePoolWithConfig<'info> {
    #[account(
        mut,
        seeds = [Global::SEED_PREFIX.as_bytes()],
        constraint = global.initialized == true @ ContractError::NotInitialized,
        bump,
    )]
    global: Box<Account<'info, Global>>,

    #[account(
        mut,
        seeds = [BondingCurve::SEED_PREFIX.as_bytes(), token_b_mint.to_account_info().key.as_ref()],
        bump,
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(mut)]
    /// CHECK: Fee receiver will be asserted in handler
    pub fee_receiver: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Pool account (PDA address)
    pub pool: UncheckedAccount<'info>,

    /// CHECK: Config for fee
    pub config: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: lp mint
    pub lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Token A LP
    pub a_vault_lp: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Token A LP
    pub b_vault_lp: UncheckedAccount<'info>,

    /// CHECK: Token A mint
    pub token_a_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_b_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    /// CHECK: Vault accounts for token A
    pub a_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Vault accounts for token B
    pub b_vault: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT_SEED.as_bytes(), a_vault.key.as_ref()],
        bump,
        seeds::program = vault_program.key()
    )]
    pub a_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT_SEED.as_bytes(), b_vault.key.as_ref()],
        bump,
        seeds::program = vault_program.key()
    )]
    pub b_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: Vault LP accounts and mints for token A
    pub a_vault_lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Vault LP accounts and mints for token B
    pub b_vault_lp_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = bonding_curve
    )]
    pub bonding_curve_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = fee_receiver
    )]
    pub fee_receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, seeds = [BondingCurve::SOL_ESCROW_SEED_PREFIX.as_bytes(), token_b_mint.to_account_info().key.as_ref()], bump)]
    /// CHECK: PDA to hold SOL for bonding curve
    pub bonding_curve_sol_escrow: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = bonding_curve_sol_escrow
    )]
    pub payer_token_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = bonding_curve_sol_escrow
    )]
    pub payer_token_b: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: Accounts to bootstrap the pool with initial liquidity
    pub payer_pool_lp: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Protocol fee token a accounts
    pub protocol_token_a_fee: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Protocol fee token b accounts
    pub protocol_token_b_fee: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: LP mint metadata PDA. Metaplex do the checking.
    pub mint_metadata: UncheckedAccount<'info>,
    /// CHECK: Additional program accounts
    pub rent: UncheckedAccount<'info>,
    /// CHECK: Metadata program account
    pub metadata_program: UncheckedAccount<'info>,
    /// CHECK: Vault program account
    pub vault_program: UncheckedAccount<'info>,
    /// CHECK: Token program account
    pub token_program: Program<'info, Token>,
    /// CHECK: Associated token program account
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: System program account
    system_program: Program<'info, System>,

    #[account(mut)]
    /// CHECK: Meteora Program
    pub meteora_program: AccountInfo<'info>,
}

pub fn initialize_pool_with_config(ctx: Context<InitializePoolWithConfig>) -> Result<()> {
    let quote_mint: Pubkey = Pubkey::from_str(QUOTE_MINT).unwrap();

    require!(
        !ctx.accounts.global.is_config_outdated()?,
        ContractError::ConfigOutdated
    );

    require!(
        ctx.accounts.bonding_curve.mint.key() == ctx.accounts.token_b_mint.key(),
        ContractError::NotBondingCurveMint
    );

    require!(
        quote_mint.key() == ctx.accounts.token_a_mint.key(),
        ContractError::NotSOL
    );

    require!(
        ctx.accounts.fee_receiver.key() == ctx.accounts.global.fee_receiver.key(),
        ContractError::InvalidFeeReceiver
    );

    require!(
        ctx.accounts.global.meteora_config.key() == ctx.accounts.config.key(),
        ContractError::InvalidConfig
    );

    require!(
        ctx.accounts.payer.key() == ctx.accounts.global.migration_authority.key(),
        ContractError::InvalidMigrationAuthority
    );
    require!(
        ctx.accounts.bonding_curve.complete,
        ContractError::NotCompleted
    );

    require!(
        ctx.accounts.meteora_program.key() == Pubkey::from_str(METEORA_PROGRAM_KEY).unwrap(),
        ContractError::InvalidMeteoraProgram
    );

    let mint_b = ctx.accounts.token_b_mint.key();

    let bonding_curve_signer = BondingCurve::get_signer(&ctx.bumps.bonding_curve, &mint_b);
    let bonding_curve_signer_seeds = &[&bonding_curve_signer[..]];

    let sol_escrow_signer =
        BondingCurve::get_sol_escrow_signer(&ctx.bumps.bonding_curve_sol_escrow, &mint_b);
    let sol_escrow_signer_seeds = &[&sol_escrow_signer[..]];

    let token_a_amount = ctx
        .accounts
        .bonding_curve
        .real_sol_reserves
        .checked_sub(ctx.accounts.global.migrate_fee_amount)
        .ok_or(ContractError::ArithmeticError)?
        .checked_sub(40_000_000)
        .ok_or(ContractError::ArithmeticError)?;

    msg!("Token A Amount: {}", token_a_amount);

    let token_b_amount = ctx
        .accounts
        .bonding_curve
        .token_total_supply
        .checked_sub(ctx.accounts.global.initial_real_token_reserves)
        .ok_or(ContractError::ArithmeticError)?
        .checked_sub(ctx.accounts.global.migration_token_allocation)
        .ok_or(ContractError::ArithmeticError)?;

    msg!("Token B Amount: {}", token_b_amount);

    // Unlock Ata before transfer
    let locker: &mut BondingCurveLockerCtx = &mut ctx
        .accounts
        .into_bonding_curve_locker_ctx(ctx.bumps.bonding_curve);
    locker.unlock_ata()?;

    // Transfer Mint B to payer token b - Bonding Curve is Signer
    let cpi_accounts = Transfer {
        from: ctx.accounts.bonding_curve_token_account.to_account_info(),
        to: ctx.accounts.payer_token_b.to_account_info(),
        authority: ctx.accounts.bonding_curve.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            bonding_curve_signer_seeds,
        ),
        token_b_amount,
    )?;

    // Transfer and wrap sol to payer token a - Sol Escrow is Signer
    // Transfer
    let sol_ix = system_instruction::transfer(
        &ctx.accounts.bonding_curve_sol_escrow.to_account_info().key,
        &ctx.accounts.payer_token_a.to_account_info().key,
        token_a_amount,
    );

    invoke_signed(
        &sol_ix,
        &[
            ctx.accounts
                .bonding_curve_sol_escrow
                .to_account_info()
                .clone(),
            ctx.accounts.payer_token_a.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        sol_escrow_signer_seeds,
    )?;

    // Sync Native mint ATA
    let cpi_accounts = token::SyncNative {
        account: ctx.accounts.payer_token_a.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, sol_escrow_signer_seeds);
    token::sync_native(cpi_ctx)?;

    // Create pool
    let mut accounts = vec![
        AccountMeta::new(ctx.accounts.pool.key(), false),
        AccountMeta::new_readonly(ctx.accounts.config.key(), false),
        AccountMeta::new(ctx.accounts.lp_mint.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_a_mint.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_b_mint.key(), false),
        AccountMeta::new(ctx.accounts.a_vault.key(), false),
        AccountMeta::new(ctx.accounts.b_vault.key(), false),
        AccountMeta::new(ctx.accounts.a_token_vault.key(), false),
        AccountMeta::new(ctx.accounts.b_token_vault.key(), false),
        AccountMeta::new(ctx.accounts.a_vault_lp_mint.key(), false),
        AccountMeta::new(ctx.accounts.b_vault_lp_mint.key(), false),
        AccountMeta::new(ctx.accounts.a_vault_lp.key(), false),
        AccountMeta::new(ctx.accounts.b_vault_lp.key(), false),
        AccountMeta::new(ctx.accounts.payer_token_a.key(), false),
        AccountMeta::new(ctx.accounts.payer_token_b.key(), false),
        AccountMeta::new(ctx.accounts.payer_pool_lp.key(), false),
        AccountMeta::new(ctx.accounts.protocol_token_a_fee.key(), false),
        AccountMeta::new(ctx.accounts.protocol_token_b_fee.key(), false),
        AccountMeta::new(ctx.accounts.bonding_curve_sol_escrow.key(), true), // Bonding Curve is the sol_escrow
        AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
        AccountMeta::new(ctx.accounts.mint_metadata.key(), false),
        AccountMeta::new_readonly(ctx.accounts.metadata_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.vault_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
    ];

    accounts.extend(ctx.remaining_accounts.iter().map(|acc| AccountMeta {
        pubkey: *acc.key,
        is_signer: false,
        is_writable: true,
    }));

    let data = get_pool_create_ix_data(token_a_amount, token_b_amount);

    let instruction = Instruction {
        program_id: ctx.accounts.meteora_program.key(),
        accounts,
        data,
    };

    invoke_signed(
        &instruction,
        &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.config.to_account_info(),
            ctx.accounts.lp_mint.to_account_info(),
            ctx.accounts.token_a_mint.to_account_info(),
            ctx.accounts.token_b_mint.to_account_info(),
            ctx.accounts.a_vault.to_account_info(),
            ctx.accounts.b_vault.to_account_info(),
            ctx.accounts.a_token_vault.to_account_info(),
            ctx.accounts.b_token_vault.to_account_info(),
            ctx.accounts.a_vault_lp_mint.to_account_info(),
            ctx.accounts.b_vault_lp_mint.to_account_info(),
            ctx.accounts.a_vault_lp.to_account_info(),
            ctx.accounts.b_vault_lp.to_account_info(),
            ctx.accounts.payer_token_a.to_account_info(),
            ctx.accounts.payer_token_b.to_account_info(),
            ctx.accounts.payer_pool_lp.to_account_info(),
            ctx.accounts.protocol_token_a_fee.to_account_info(),
            ctx.accounts.protocol_token_b_fee.to_account_info(),
            ctx.accounts.bonding_curve_sol_escrow.to_account_info(), // Signer is the SOL Escrow
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.mint_metadata.to_account_info(),
            ctx.accounts.metadata_program.to_account_info(),
            ctx.accounts.vault_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        sol_escrow_signer_seeds, // Signer is the SOL Escrow
    )?;

    // Fee transfer
    let sol_ix = system_instruction::transfer(
        &ctx.accounts.bonding_curve_sol_escrow.to_account_info().key,
        &ctx.accounts.fee_receiver.to_account_info().key,
        ctx.accounts.global.migrate_fee_amount,
    );

    invoke_signed(
        &sol_ix,
        &[
            ctx.accounts
                .bonding_curve_sol_escrow
                .to_account_info()
                .clone(),
            ctx.accounts.fee_receiver.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        sol_escrow_signer_seeds,
    )?;

    // Transfer community allocation to fee_receiver
    let cpi_accounts = Transfer {
        from: ctx.accounts.bonding_curve_token_account.to_account_info(),
        to: ctx.accounts.fee_receiver_token_account.to_account_info(),
        authority: ctx.accounts.bonding_curve.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            bonding_curve_signer_seeds,
        ),
        ctx.accounts.global.migration_token_allocation,
    )?;

    locker.revoke_freeze_authority()?;
    Ok(())
}

impl<'info> IntoBondingCurveLockerCtx<'info> for InitializePoolWithConfig<'info> {
    fn into_bonding_curve_locker_ctx(
        &self,
        bonding_curve_bump: u8,
    ) -> BondingCurveLockerCtx<'info> {
        BondingCurveLockerCtx {
            bonding_curve_bump,
            mint: self.token_b_mint.clone(),
            bonding_curve: self.bonding_curve.clone(),
            bonding_curve_sol_escrow: self.bonding_curve_sol_escrow.clone(),
            bonding_curve_token_account: self.bonding_curve_token_account.clone(),
            token_program: self.token_program.clone(),
            global: self.global.clone(),
        }
    }
}
