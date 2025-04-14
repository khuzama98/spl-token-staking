use anchor_lang::prelude::*;
use crate::accounts_module::initialize::Initialize;

pub fn handler(ctx: Context<Initialize<'_>>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;

    global_state.total_staked = 0;
    global_state.admin = ctx.accounts.admin.key();
    global_state.staking_token_mint = ctx.accounts.mint.key();
    global_state.vault = ctx.accounts.vault.key();
    global_state.vault_bump = ctx.bumps.vault_authority;

    msg!("Staking account initialized");
    Ok(())
}