use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub admin: Pubkey,
    pub staking_token_mint: Pubkey,
    pub vault: Pubkey,
    pub vault_bump: u8,
    pub total_staked: u64,
}