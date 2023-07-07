use anchor_lang::prelude::*;

use anchor_spl::{
    token::{Mint, Token},
};


use crate::state::*;

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    #[account(
        init,
        payer = from,
        space = 8 + 8 + 64,
    )]
    pub vault: Account<'info, Vault>,

    // Mint account
    #[account(
        init,
        seeds = [b"vault_mint".as_ref()],
        bump,
        payer = from,
        mint::decimals = 9,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}


pub fn handler(
    ctx: Context<CreateVault>,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let from = &mut ctx.accounts.from;
    vault.owner = *from.key;
    vault.stable_amount = 0;
    vault.sol_amount = 0;
    vault.allow_withdrawal = vec![];
    Ok(())
}