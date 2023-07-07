use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    // The account that owns the vault.
    pub owner: Pubkey,
    // The amount of stable tokens in the vault.
    pub stable_amount: u64,
    // The amount of SOL in the vault
    pub sol_amount: u64,
    // The accounts that can withdraw from the vault.
    pub allow_withdrawal: Vec<Pubkey>,
}