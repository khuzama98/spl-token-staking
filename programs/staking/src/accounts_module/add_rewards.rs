use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::state::global_state::GlobalState;

#[derive(Accounts)]
pub struct AddRewards<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [b"global_state"], bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut, seeds = [b"reward_pool"], bump)]
    pub reward_pool: Account<'info, TokenAccount>,
    #[account(seeds = [b"reward_pool_authority"], bump)]
    pub reward_pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}