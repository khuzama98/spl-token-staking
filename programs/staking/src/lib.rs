use anchor_lang::prelude::*;

pub mod instructions;
pub mod accounts_module;
pub mod state;

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize<'_>>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake<'_>>, amount: u64) -> Result<()> {
        instructions::stake::handler(ctx, amount)
    }
}
