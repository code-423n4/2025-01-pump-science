use crate::{
    errors::ContractError,
    events::{GlobalUpdateEvent, IntoEvent},
};
use anchor_lang::{prelude::*, solana_program::last_restart_slot::LastRestartSlot};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GlobalAuthorityInput {
    pub global_authority: Option<Pubkey>,
    pub migration_authority: Option<Pubkey>,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Global {
    pub initialized: bool,
    pub global_authority: Pubkey,    // can update settings
    pub migration_authority: Pubkey, // can migrate
    pub migrate_fee_amount: u64,
    pub migration_token_allocation: u64,
    pub fee_receiver: Pubkey,

    // Pump.science initial values
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub mint_decimals: u8,

    pub meteora_config: Pubkey,
    pub whitelist_enabled: bool,

    pub last_updated_slot: u64,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            initialized: true,
            global_authority: Pubkey::default(),
            migration_authority: Pubkey::default(),
            fee_receiver: Pubkey::default(),
            // Pump.fun initial values
            initial_virtual_token_reserves: 1073000000000000,
            initial_virtual_sol_reserves: 30000000000,
            initial_real_token_reserves: 793100000000000,
            token_total_supply: 1000000000000000,
            mint_decimals: 6,
            migrate_fee_amount: 500,
            migration_token_allocation: 50000000000000, // 50M
            whitelist_enabled: true,
            meteora_config: Pubkey::default(),
            last_updated_slot: 0,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct GlobalSettingsInput {
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub mint_decimals: u8,
    pub migrate_fee_amount: u64,
    pub migration_token_allocation: u64,
    pub fee_receiver: Pubkey,
    pub whitelist_enabled: bool,
    pub meteora_config: Pubkey,
}

impl Global {
    pub const SEED_PREFIX: &'static str = "global";

    pub fn get_signer<'a>(bump: &'a u8) -> [&'a [u8]; 2] {
        let prefix_bytes = Self::SEED_PREFIX.as_bytes();
        let bump_slice: &'a [u8] = std::slice::from_ref(bump);
        [prefix_bytes, bump_slice]
    }

    pub fn validate_settings(&self, params: &GlobalSettingsInput) -> Result<()> {
        require!(params.mint_decimals <= 9, ContractError::InvalidParameter);
        require!(
            params.token_total_supply <= u64::MAX / 2,
            ContractError::InvalidParameter
        );
        require!(
            params.fee_receiver != Pubkey::default(),
            ContractError::InvalidParameter
        );
        require!(
            params.meteora_config != Pubkey::default(),
            ContractError::InvalidParameter
        );
        require!(
            params.initial_virtual_token_reserves > 0,
            ContractError::InvalidParameter
        );
        require!(
            params.initial_virtual_sol_reserves > 0,
            ContractError::InvalidParameter
        );
        require!(
            params.initial_real_token_reserves > 0,
            ContractError::InvalidParameter
        );

        // Making sure that there is a token amount left for the real token reserves
        require!(
            params
                .token_total_supply
                .saturating_sub(params.migration_token_allocation)
                > params.initial_real_token_reserves,
            ContractError::InvalidParameter
        );
        Ok(())
    }

    pub fn update_settings(&mut self, params: GlobalSettingsInput, slot: u64) {
        self.mint_decimals = params.mint_decimals;
        self.initial_virtual_token_reserves = params.initial_virtual_token_reserves;
        self.initial_virtual_sol_reserves = params.initial_virtual_sol_reserves;
        self.initial_real_token_reserves = params.initial_real_token_reserves;
        self.token_total_supply = params.token_total_supply;
        self.migrate_fee_amount = params.migrate_fee_amount;
        self.fee_receiver = params.fee_receiver;
        self.whitelist_enabled = params.whitelist_enabled;
        self.meteora_config = params.meteora_config;

        // Set last updated slot to the slot of the update
        self.last_updated_slot = slot;
    }

    pub fn update_authority(&mut self, params: GlobalAuthorityInput) {
        if let Some(global_authority) = params.global_authority {
            self.global_authority = global_authority;
        }
        if let Some(migration_authority) = params.migration_authority {
            self.migration_authority = migration_authority;
        }
    }

    pub fn is_config_outdated(&self) -> Result<bool> {
        let last_restart_slot = LastRestartSlot::get()?;
        Ok(self.last_updated_slot <= last_restart_slot.last_restart_slot)
    }
}

impl IntoEvent<GlobalUpdateEvent> for Global {
    fn into_event(&self) -> GlobalUpdateEvent {
        GlobalUpdateEvent {
            global_authority: self.global_authority,
            migration_authority: self.migration_authority,
            initial_virtual_token_reserves: self.initial_virtual_token_reserves,
            initial_virtual_sol_reserves: self.initial_virtual_sol_reserves,
            initial_real_token_reserves: self.initial_real_token_reserves,
            token_total_supply: self.token_total_supply,
            mint_decimals: self.mint_decimals,
        }
    }
}
