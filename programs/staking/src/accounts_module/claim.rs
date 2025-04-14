use anchor_lang::prelude::*;
use crate::state::{global_state::GlobalState, staker::Staker};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"staker", signer.key().as_ref()], bump)]
    pub staker: Account<'info, Staker>,
    #[account(mut, seeds = [b"global_state"], bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut, seeds = [b"reward_pool"], bump)]
    pub reward_pool: Account<'info, TokenAccount>, // Reward pool account
    #[account(seeds = [b"reward_pool_authority"], bump)]
    pub reward_pool_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}