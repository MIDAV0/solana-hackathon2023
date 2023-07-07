use anchor_lang::{prelude::*, solana_program::system_instruction};


use crate::state::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount, transfer, Transfer, Mint},
};

#[derive(Accounts)]
pub struct BorrowFromVault<'info> {

    #[account(mut)]
    pub from: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = mint,
        associated_token::authority = from,
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(
    ctx: Context<BorrowFromVault>,
    amount: u64,
) -> Result<()> {
    let from = &ctx.accounts.from;
    let vault = &mut ctx.accounts.vault;

    // Check if the vault is allowed to be withdrawn from
    // if !vault.allow_withdrawal.contains(&from.key) {
    //     return Err(ErrorCode::Unauthorized.into());
    // }

    // Create transfer instruction
    let transfer_instruction = system_instruction::transfer(
        from.key,
        vault.to_account_info().key,
        amount
    );

    // Invoke transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    // Transfer stable tokens from vault to from_token_account
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info().clone(),
        to: ctx.accounts.from_token_account.to_account_info().clone(),
        authority: ctx.accounts.vault.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_ctx, amount)?;

    Ok(())
}