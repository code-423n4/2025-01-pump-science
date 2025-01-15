use crate::errors::ContractError;
use crate::state::bonding_curve::locker::BondingCurveLockerCtx;
use crate::state::bonding_curve::*;
use crate::util::bps_mul;
use crate::Global;
use anchor_lang::prelude::*;
use std::fmt::{self};
use structs::BondingCurve;

impl BondingCurve {
    pub const SEED_PREFIX: &'static str = "bonding-curve";
    pub const SOL_ESCROW_SEED_PREFIX: &'static str = "sol-escrow";
    pub fn calculate_fee(&self, amount: u64, current_slot: u64) -> Result<u64> {
        let start_slot = self.start_slot;

        msg!("Start slot: {}", start_slot);
        msg!("Current slot: {}", current_slot);

        let slots_passed = current_slot - start_slot;
        msg!("Slots passed: {}", slots_passed);

        let mut sol_fee: u64 = 0;

        if slots_passed < 150 {
            msg!("Phase 1: 99% fees between slot 0 - 150");
            sol_fee = bps_mul(9900, amount, 10_000).unwrap();
        } else if slots_passed >= 150 && slots_passed <= 250 {
            msg!("Phase 2: Linear decrease between 150 - 250");

            // Calculate the minimum fee bps (at slot 250) scaled by 100_000 for precision
            let fee_bps = (-8_300_000_i64)
                .checked_mul(slots_passed as i64)
                .ok_or(ContractError::ArithmeticError)?
                .checked_add(2_162_600_000)
                .ok_or(ContractError::ArithmeticError)?
                .checked_div(100_000)
                .ok_or(ContractError::ArithmeticError)?;
            msg!("Fee Bps: {}", fee_bps);

            sol_fee = bps_mul(fee_bps as u64, amount, 10_000).unwrap();
        } else if slots_passed > 250 {
            msg!("Phase 3: 1% fees after 250");
            sol_fee = bps_mul(100, amount, 10_000).unwrap();
        }
        Ok(sol_fee)
    }

    pub fn get_signer<'a>(bump: &'a u8, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [
            Self::SEED_PREFIX.as_bytes(),
            mint.as_ref(),
            std::slice::from_ref(bump),
        ]
    }

    pub fn get_sol_escrow_signer<'a>(bump: &'a u8, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [
            Self::SOL_ESCROW_SEED_PREFIX.as_bytes(),
            mint.as_ref(),
            std::slice::from_ref(bump),
        ]
    }

    pub fn update_from_params(
        &mut self,
        mint: Pubkey,
        creator: Pubkey,
        global_config: &Global,
        params: &CreateBondingCurveParams,
        clock: &Clock,
        bump: u8,
    ) -> &mut Self {
        let start_slot = if let Some(start_slot) = params.start_slot {
            start_slot
        } else {
            clock.slot
        };
        let creator = creator;
        let complete = false;
        self.clone_from(&BondingCurve {
            mint,
            creator,
            virtual_token_reserves: global_config.initial_virtual_token_reserves,
            virtual_sol_reserves: global_config.initial_virtual_sol_reserves,
            initial_real_token_reserves: global_config.initial_real_token_reserves,
            real_sol_reserves: 0,
            real_token_reserves: global_config.initial_real_token_reserves,
            token_total_supply: global_config.token_total_supply,
            start_slot,
            complete,
            bump,
        });
        self
    }

    pub fn apply_buy(&mut self, mut sol_amount: u64) -> Option<BuyResult> {
        msg!("ApplyBuy: sol_amount: {}", sol_amount);

        // Computing Token Amount out
        let mut token_amount = self.get_tokens_for_buy_sol(sol_amount)?;
        msg!("ApplyBuy: token_amount: {}", token_amount);

        if token_amount >= self.real_token_reserves {
            // Last Buy
            token_amount = self.real_token_reserves;

            // Temporarily store the current state
            let current_virtual_token_reserves = self.virtual_token_reserves;
            let current_virtual_sol_reserves = self.virtual_sol_reserves;

            // Update self with the new token amount
            self.virtual_token_reserves = (current_virtual_token_reserves as u128)
                .checked_sub(token_amount as u128)?
                .try_into()
                .ok()?;
            self.virtual_sol_reserves = 115_005_359_056; // Total raise amount at end

            let recomputed_sol_amount = self.get_sol_for_sell_tokens(token_amount)?;
            msg!("ApplyBuy: recomputed_sol_amount: {}", recomputed_sol_amount);
            sol_amount = recomputed_sol_amount;

            // Restore the state with the recomputed sol_amount
            self.virtual_token_reserves = current_virtual_token_reserves;
            self.virtual_sol_reserves = current_virtual_sol_reserves;

            // Set complete to true
            self.complete = true;
        }

        // Adjusting token reserve values
        // New Virtual Token Reserves
        let new_virtual_token_reserves =
            (self.virtual_token_reserves as u128).checked_sub(token_amount as u128)?;
        msg!(
            "ApplyBuy: new_virtual_token_reserves: {}",
            new_virtual_token_reserves
        );

        // New Real Token Reserves
        let new_real_token_reserves =
            (self.real_token_reserves as u128).checked_sub(token_amount as u128)?;
        msg!(
            "ApplyBuy: new_real_token_reserves: {}",
            new_real_token_reserves
        );

        // Adjusting sol reserve values
        // New Virtual Sol Reserves
        let new_virtual_sol_reserves =
            (self.virtual_sol_reserves as u128).checked_add(sol_amount as u128)?;
        msg!(
            "ApplyBuy: new_virtual_sol_reserves: {}",
            new_virtual_sol_reserves
        );

        // New Real Sol Reserves
        let new_real_sol_reserves =
            (self.real_sol_reserves as u128).checked_add(sol_amount as u128)?;
        msg!("ApplyBuy: new_real_sol_reserves: {}", new_real_sol_reserves);

        self.virtual_token_reserves = new_virtual_token_reserves.try_into().ok()?;
        self.real_token_reserves = new_real_token_reserves.try_into().ok()?;
        self.virtual_sol_reserves = new_virtual_sol_reserves.try_into().ok()?;
        self.real_sol_reserves = new_real_sol_reserves.try_into().ok()?;
        self.msg();
        Some(BuyResult {
            token_amount,
            sol_amount,
        })
    }

    pub fn apply_sell(&mut self, token_amount: u64) -> Option<SellResult> {
        msg!("apply_sell: token_amount: {}", token_amount);

        // Computing Sol Amount out
        let sol_amount = self.get_sol_for_sell_tokens(token_amount)?;
        msg!("apply_sell: sol_amount: {}", sol_amount);

        // Adjusting token reserve values
        // New Virtual Token Reserves
        let new_virtual_token_reserves =
            (self.virtual_token_reserves as u128).checked_add(token_amount as u128)?;
        msg!(
            "apply_sell: new_virtual_token_reserves: {}",
            new_virtual_token_reserves
        );

        // New Real Token Reserves
        let new_real_token_reserves =
            (self.real_token_reserves as u128).checked_add(token_amount as u128)?;
        msg!(
            "apply_sell: new_real_token_reserves: {}",
            new_real_token_reserves
        );

        // Adjusting sol reserve values
        // New Virtual Sol Reserves
        let new_virtual_sol_reserves =
            (self.virtual_sol_reserves as u128).checked_sub(sol_amount as u128)?;
        msg!(
            "apply_sell: new_virtual_sol_reserves: {}",
            new_virtual_sol_reserves
        );

        // New Real Sol Reserves
        let new_real_sol_reserves = self.real_sol_reserves.checked_sub(sol_amount)?;
        msg!(
            "apply_sell: new_real_sol_reserves: {}",
            new_real_sol_reserves
        );

        self.virtual_token_reserves = new_virtual_token_reserves.try_into().ok()?;
        self.real_token_reserves = new_real_token_reserves.try_into().ok()?;
        self.virtual_sol_reserves = new_virtual_sol_reserves.try_into().ok()?;
        self.real_sol_reserves = new_real_sol_reserves.try_into().ok()?;
        self.msg();
        Some(SellResult {
            token_amount,
            sol_amount,
        })
    }

    pub fn get_sol_for_sell_tokens(&self, token_amount: u64) -> Option<u64> {
        if token_amount == 0 {
            return None;
        }
        msg!("GetSolForSellTokens: token_amount: {}", token_amount);

        // Convert to common decimal basis (using 9 decimals as base)
        let current_sol = self.virtual_sol_reserves as u128;
        let current_tokens = (self.virtual_token_reserves as u128)
            .checked_mul(1_000_000_000)? // Scale tokens up to 9 decimals
            .checked_div(1_000_000)?; // From 6 decimals

        // Calculate new reserves using constant product formula
        let new_tokens = current_tokens.checked_add(
            (token_amount as u128)
                .checked_mul(1_000_000_000)? // Scale input tokens to 9 decimals
                .checked_div(1_000_000)?, // From 6 decimals
        )?;

        let new_sol = (current_sol.checked_mul(current_tokens)?).checked_div(new_tokens)?;

        let sol_out = current_sol.checked_sub(new_sol)?;

        msg!("GetSolForSellTokens: sol_out: {}", sol_out);
        <u128 as TryInto<u64>>::try_into(sol_out).ok()
    }

    pub fn get_tokens_for_buy_sol(&self, sol_amount: u64) -> Option<u64> {
        if sol_amount == 0 {
            return None;
        }
        msg!("GetTokensForBuySol: sol_amount: {}", sol_amount);

        // Convert to common decimal basis (using 9 decimals as base)
        let current_sol = self.virtual_sol_reserves as u128;
        let current_tokens = (self.virtual_token_reserves as u128)
            .checked_mul(1_000_000_000)? // Scale tokens up to 9 decimals
            .checked_div(1_000_000)?; // From 6 decimals

        // Calculate new reserves using constant product formula
        let new_sol = current_sol.checked_add(sol_amount as u128)?;
        let new_tokens = (current_sol.checked_mul(current_tokens)?).checked_div(new_sol)?;

        let tokens_out = current_tokens.checked_sub(new_tokens)?;

        // Convert back to 6 decimal places for tokens
        let tokens_out = tokens_out
            .checked_mul(1_000_000)? // Convert to 6 decimals
            .checked_div(1_000_000_000)?; // From 9 decimals

        msg!("GetTokensForBuySol: tokens_out: {}", tokens_out);
        <u128 as TryInto<u64>>::try_into(tokens_out).ok()
    }

    pub fn is_started(&self, clock: &Clock) -> bool {
        let now = clock.slot;
        now >= self.start_slot
    }

    pub fn msg(&self) -> () {
        msg!("{:#?}", self);
    }

    pub fn invariant<'info>(ctx: &mut BondingCurveLockerCtx<'info>) -> Result<()> {
        let bonding_curve = &mut ctx.bonding_curve;
        let tkn_account = &mut ctx.bonding_curve_token_account;
        let sol_escrow = &mut ctx.bonding_curve_sol_escrow;

        if tkn_account.owner != bonding_curve.key() {
            msg!("Invariant failed: invalid token acc supplied");
            return Err(ContractError::BondingCurveInvariant.into());
        }
        tkn_account.reload()?;

        // let lamports = bonding_curve.get_lamports();
        let tkn_balance = tkn_account.amount;
        let sol_escrow_lamports = sol_escrow.lamports();

        // let rent_exemption_balance: u64 =
        //     Rent::get()?.minimum_balance(8 + BondingCurve::INIT_SPACE as usize);
        // let bonding_curve_pool_lamports: u64 = lamports - rent_exemption_balance;

        // Ensure real sol reserves are equal to bonding curve pool lamports
        if sol_escrow_lamports < bonding_curve.real_sol_reserves {
            msg!(
                "real_sol_r:{}, bonding_lamps:{}",
                bonding_curve.real_sol_reserves,
                sol_escrow_lamports
            );
            msg!("Invariant failed: real_sol_reserves != bonding_curve_pool_lamports");
            return Err(ContractError::BondingCurveInvariant.into());
        }

        // Ensure the virtual reserves are always positive
        if bonding_curve.virtual_sol_reserves <= 0 {
            msg!("Invariant failed: virtual_sol_reserves <= 0");
            return Err(ContractError::BondingCurveInvariant.into());
        }
        if bonding_curve.virtual_token_reserves <= 0 {
            msg!("Invariant failed: virtual_token_reserves <= 0");
            return Err(ContractError::BondingCurveInvariant.into());
        }

        // Ensure the token total supply is consistent with the reserves
        let tkn_balance_minus_bonding_liquidity = tkn_balance
            .checked_sub(
                bonding_curve
                    .token_total_supply
                    .checked_sub(bonding_curve.initial_real_token_reserves)
                    .ok_or(ContractError::ArithmeticError)?,
            )
            .ok_or(ContractError::ArithmeticError)?;

        if tkn_balance_minus_bonding_liquidity != bonding_curve.real_token_reserves {
            msg!("Invariant failed: real_token_reserves != tkn_balance");
            msg!("real_token_reserves: {}", bonding_curve.real_token_reserves);
            msg!("tkn_balance: {}", tkn_balance);
            msg!(
                "tkn_balance_minus_bonding_liquidity: {}",
                tkn_balance_minus_bonding_liquidity
            );
            return Err(ContractError::BondingCurveInvariant.into());
        }

        // Ensure the bonding curve is complete only if real token reserves are zero
        if bonding_curve.complete && bonding_curve.real_token_reserves != 0 {
            msg!("Invariant failed: bonding curve marked as complete but real_token_reserves != 0");
            return Err(ContractError::BondingCurveInvariant.into());
        }

        if !bonding_curve.complete && !tkn_account.is_frozen() {
            msg!("Active BondingCurve TokenAccount must always be frozen at the end");
            return Err(ContractError::BondingCurveInvariant.into());
        }
        Ok(())
    }
}

impl fmt::Display for BondingCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BondingCurve {{ creator: {:?}, initial_real_token_reserves: {:?}, virtual_sol_reserves: {:?}, virtual_token_reserves: {:?}, real_sol_reserves: {:?}, real_token_reserves: {:?}, token_total_supply: {:?}, start_slot: {:?}, complete: {:?} }}",
            self.creator,
            self.initial_real_token_reserves,
            self.virtual_sol_reserves,
            self.virtual_token_reserves,
            self.real_sol_reserves,
            self.real_token_reserves,
            self.token_total_supply,
            self.start_slot,
            self.complete
        )
    }
}
