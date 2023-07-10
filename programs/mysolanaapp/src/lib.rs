use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("H2uuip7vMqreJJfYPMvpAttqwxuEurpCr7RnrhXFvUtx");

// Processor
#[program]
pub mod mysolanaapp {
    use super::*;


    pub fn create_vault(ctx: Context<CreateVault>, vault_address: Pubkey) -> Result<()> {
        instructions::create_vault::handler(ctx)
    }

    pub fn deposit_to_vault(
        ctx: Context<DepositToVault>, 
        stable_token_bump: u8,
        vault_token_bump: u8, 
        vault_token_mint_address: Pubkey,
        stable_token_mint_address: Pubkey,
        deposit_amount: u64,
    ) -> Result<()> {
        instructions::deposit_to_vault::handler(
            ctx, 
            stable_token_bump,
            vault_token_bump,
            deposit_amount
        )
    }

    pub fn withdraw_from_vault(
        ctx: Context<WithdrawFromVault>, 
        vault_stable_token_bump: u8,
        vault_token_mint_address: Pubkey,
        stable_token_mint_address: Pubkey,
        withdraw_amount: u64
    ) -> Result<()> {
        instructions::withdraw_from_vault::handler(
            ctx,
            vault_stable_token_bump,
            withdraw_amount
        )
    }

    pub fn borrow_from_vault(ctx: Context<BorrowFromVault>, amount: u64) -> Result<()> {
        instructions::borrow_from_vault::handler(ctx, amount)
    }


}