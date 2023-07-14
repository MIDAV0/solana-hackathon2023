use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    // The account that owns the vault.
    pub vault_owner: Pubkey,
    // The amount of stable tokens in the vault.
    pub stable_amount: u64,
    // The amount of SOL in the vault
    pub sol_amount: u64,

    // Vault token supply
    pub vault_token_supply: u64,
    // Vault start time
    pub start_time: u64,
    // Vault end time
    pub end_time: u64,
    // Vault borrowing start time
    pub borrowing_start_time: u64,
    // Vault borrowing end time
    pub borrowing_end_time: u64,
    pub rewards: u64,

}