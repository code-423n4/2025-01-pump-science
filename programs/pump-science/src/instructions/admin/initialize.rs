use std::str::FromStr;

use crate::{
    constants::CREATION_AUTHORITY_PUBKEY, errors::ContractError, events::*, state::global::*,
};
use anchor_lang::prelude::*;

#[event_cpi]
#[derive(Accounts)]
#[instruction(params: GlobalSettingsInput)]
pub struct Initialize<'info> {
    #[account(mut, constraint = authority.key() == Pubkey::from_str(CREATION_AUTHORITY_PUBKEY).unwrap() @ ContractError::InvalidAuthority)]
    authority: Signer<'info>,

    #[account(
        init,
        space = 8 + Global::INIT_SPACE,
        seeds = [Global::SEED_PREFIX.as_bytes()],
        constraint = global.initialized != true @ ContractError::AlreadyInitialized,
        bump,
        payer = authority,
    )]
    global: Box<Account<'info, Global>>,

    system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn handler(ctx: Context<Initialize>, params: GlobalSettingsInput) -> Result<()> {
        let global = &mut ctx.accounts.global;

        // Validate parameters
        global.validate_settings(&params)?;

        global.update_authority(GlobalAuthorityInput {
            global_authority: Some(ctx.accounts.authority.key()),
            migration_authority: Some(ctx.accounts.authority.key()),
        });
        global.update_settings(params.clone(), Clock::get()?.slot);

        require_gt!(global.mint_decimals, 0, ContractError::InvalidArgument);

        global.initialized = true;
        emit_cpi!(global.into_event());
        Ok(())
    }
}
