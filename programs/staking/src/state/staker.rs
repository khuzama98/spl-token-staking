use anchor_lang::prelude::*;

#[account]
pub struct Staker {
    pub address: Pubkey,
    pub staked_amount: u64,
}