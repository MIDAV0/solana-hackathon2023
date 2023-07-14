use anchor_lang::prelude::*;

use anchor_spl::{
    token::{Mint, Token, TokenAccount},
};


use crate::state::*;

#[derive(Accounts)]
#[instruction(stable_token_mint_address: Pubkey)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    // Vault account
    #[account(
        init,
        payer = from,
        space = 64+64,
    )]
    pub vault: Account<'info, Vault>,

    // Vault stable token PDA
    #[account(
        init,
        payer = from,
        seeds = [b"stable-token-mint".as_ref()],
        bump,
        token::mint = stable_token_mint,
        token::authority = vault_stable_token_account,
    )]
    pub vault_stable_token_account: Account<'info, TokenAccount>,

    // Stable token Mint account
    #[account(
        address = stable_token_mint_address,
    )]
    pub stable_token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn handler(
    ctx: Context<CreateVault>,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let from = &mut ctx.accounts.from;
    vault.vault_owner = *from.key;
    vault.stable_amount = 0;
    vault.sol_amount = 0;
    Ok(())
}