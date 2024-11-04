use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct OhmInitialize<'info>{
    #[account(mut)]
    pub admin: Signer<'info>
}

pub fn ohm_initialize(ctx: Context<OhmInitialize>) -> Result<()>{
    Ok(())
}