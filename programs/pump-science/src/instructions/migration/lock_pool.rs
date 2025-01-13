use crate::constants::METEORA_PROGRAM_KEY;
use crate::errors::ContractError;
use crate::state::bonding_curve::BondingCurve;
use crate::state::meteora::{get_function_hash, get_lock_lp_ix_data};
use crate::Global;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
use anchor_spl::{associated_token, token::TokenAccount};
use std::str::FromStr;

#[derive(Accounts)]
pub struct LockPool<'info> {
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

    #[account(mut, seeds = [BondingCurve::SOL_ESCROW_SEED_PREFIX.as_bytes(), token_b_mint.to_account_info().key.as_ref()], bump)]
    /// CHECK: PDA to hold SOL for bonding curve
    pub bonding_curve_sol_escrow: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: Pool account (PDA address)
    pub pool: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: lp mint
    pub lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Token A LP
    pub a_vault_lp: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Token A LP
    pub b_vault_lp: UncheckedAccount<'info>,

    /// CHECK: Token B mint
    pub token_b_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Vault accounts for token A
    pub a_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Vault accounts for token B
    pub b_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Vault LP accounts and mints for token A
    pub a_vault_lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Vault LP accounts and mints for token B
    pub b_vault_lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Accounts to bootstrap the pool with initial liquidity
    pub payer_pool_lp: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: Fee receiver
    pub fee_receiver: UncheckedAccount<'info>,

    /// CHECK: Token program account
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: Associated token program account
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: System program account
    pub system_program: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK lock escrow
    pub lock_escrow: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Escrow vault
    pub escrow_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK:
    pub meteora_program: AccountInfo<'info>,

    /// CHECK: Meteora Event Autority
    pub event_authority: AccountInfo<'info>,
}

pub fn lock_pool(ctx: Context<LockPool>) -> Result<()> {
    require!(
        !ctx.accounts.global.is_config_outdated()?,
        ContractError::ConfigOutdated
    );

    require!(
        ctx.accounts.payer.key() == ctx.accounts.global.migration_authority.key(),
        ContractError::InvalidMigrationAuthority
    );

    require!(
        ctx.accounts.fee_receiver.key() == ctx.accounts.global.fee_receiver.key(),
        ContractError::InvalidFeeReceiver
    );

    require!(
        ctx.accounts.meteora_program.key() == Pubkey::from_str(METEORA_PROGRAM_KEY).unwrap(),
        ContractError::InvalidMeteoraProgram
    );

    let mint_k = ctx.accounts.token_b_mint.key();

    // Bonding Curve Sol Escrow hold LP tokens and will be the payer/signer for the lock pool
    let bonding_curve_sol_escrow_signer =
        BondingCurve::get_sol_escrow_signer(&ctx.bumps.bonding_curve_sol_escrow, &mint_k);
    let bonding_curve_sol_escrow_signer_seeds = &[&bonding_curve_sol_escrow_signer[..]];

    let meteora_program_id: Pubkey = Pubkey::from_str(METEORA_PROGRAM_KEY).unwrap();
    let source_tokens = ctx.accounts.payer_pool_lp.clone();
    let lp_mint_amount = ctx.accounts.payer_pool_lp.amount;

    // Create Lock Escrow
    let escrow_accounts = vec![
        AccountMeta::new(ctx.accounts.pool.key(), false),
        AccountMeta::new(ctx.accounts.lock_escrow.key(), false),
        AccountMeta::new_readonly(ctx.accounts.fee_receiver.key(), false),
        AccountMeta::new_readonly(ctx.accounts.lp_mint.key(), false),
        AccountMeta::new(ctx.accounts.bonding_curve_sol_escrow.key(), true), // Bonding Curve Sol Escrow is the payer/signer
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
    ];

    let escrow_instruction = Instruction {
        program_id: meteora_program_id,
        accounts: escrow_accounts,
        data: get_function_hash("global", "create_lock_escrow").into(),
    };

    invoke_signed(
        &escrow_instruction,
        &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.lock_escrow.to_account_info(),
            ctx.accounts.fee_receiver.to_account_info(),
            ctx.accounts.lp_mint.to_account_info(),
            ctx.accounts.bonding_curve_sol_escrow.to_account_info(), // Bonding Curve Sol Escrow is the payer/signer
            ctx.accounts.system_program.to_account_info(),
        ],
        bonding_curve_sol_escrow_signer_seeds,
    )?;

    associated_token::create_idempotent(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.payer.to_account_info(),
            associated_token: ctx.accounts.escrow_vault.to_account_info(),
            authority: ctx.accounts.lock_escrow.to_account_info(),
            mint: ctx.accounts.lp_mint.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
    ))?;

    // Lock Pool
    let lock_accounts = vec![
        AccountMeta::new(ctx.accounts.pool.key(), false),
        AccountMeta::new_readonly(ctx.accounts.lp_mint.key(), false),
        AccountMeta::new(ctx.accounts.lock_escrow.key(), false),
        AccountMeta::new(ctx.accounts.bonding_curve_sol_escrow.key(), true), // Bonding Curve Sol Escrow is the payer/signer
        AccountMeta::new(source_tokens.key(), false),
        AccountMeta::new(ctx.accounts.escrow_vault.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.a_vault.key(), false),
        AccountMeta::new_readonly(ctx.accounts.b_vault.key(), false),
        AccountMeta::new_readonly(ctx.accounts.a_vault_lp.key(), false),
        AccountMeta::new_readonly(ctx.accounts.b_vault_lp.key(), false),
        AccountMeta::new_readonly(ctx.accounts.a_vault_lp_mint.key(), false),
        AccountMeta::new_readonly(ctx.accounts.b_vault_lp_mint.key(), false),
    ];

    let lock_instruction = Instruction {
        program_id: meteora_program_id,
        accounts: lock_accounts,
        data: get_lock_lp_ix_data(lp_mint_amount),
    };

    invoke_signed(
        &lock_instruction,
        &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.lp_mint.to_account_info(),
            ctx.accounts.lock_escrow.to_account_info(),
            ctx.accounts.bonding_curve_sol_escrow.to_account_info(), // Bonding Curve Sol Escrow is the payer/signer
            source_tokens.to_account_info(),
            ctx.accounts.escrow_vault.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.a_vault.to_account_info(),
            ctx.accounts.b_vault.to_account_info(),
            ctx.accounts.a_vault_lp.to_account_info(),
            ctx.accounts.b_vault_lp.to_account_info(),
            ctx.accounts.a_vault_lp_mint.to_account_info(),
            ctx.accounts.b_vault_lp_mint.to_account_info(),
        ],
        bonding_curve_sol_escrow_signer_seeds,
    )?;

    // Transfer all remaining lamports
    let bonding_curve_remaining_lamports = ctx.accounts.bonding_curve_sol_escrow.get_lamports();
    msg!(
        "Bonding Curve Remaining Lamports: {}",
        bonding_curve_remaining_lamports
    );

    let sol_ix = system_instruction::transfer(
        &ctx.accounts.bonding_curve_sol_escrow.to_account_info().key,
        &ctx.accounts.fee_receiver.to_account_info().key,
        bonding_curve_remaining_lamports,
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
        bonding_curve_sol_escrow_signer_seeds,
    )?;

    Ok(())
}
