use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("H2uuip7vMqreJJfYPMvpAttqwxuEurpCr7RnrhXFvUtx");

// Processor
#[program]
pub mod mysolanaapp {
    use super::*;


    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        instructions::create_vault::handler(ctx)
    }

    pub fn deposit_to_vault(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
        instructions::deposit_to_vault::handler(ctx, amount)
    }

    pub fn borrow_from_vault(ctx: Context<BorrowFromVault>, amount: u64) -> Result<()> {
        instructions::borrow_from_vault::handler(ctx, amount)
    }

}