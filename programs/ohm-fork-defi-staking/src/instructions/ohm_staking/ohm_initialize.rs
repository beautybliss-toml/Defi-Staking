use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

use crate::errors::ErrorCode;
use crate::{ADMIN, OHM_FORK_MINT_ADDRESS};

#[derive(Accounts)]
pub struct OhmInitialize<'info>{
    /// CHECK
    #[account(
        init_if_needed,
        payer=admin,
        seeds=[b"staking_pda"],
        bump,
        space=8
    )]
    staking_pda: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer=admin,
        seeds=[b"staking_pda_ata", OHM_FORK_MINT_ADDRESS.as_bytes()],
        token::mint=staking_token_mint,
        token::authority=staking_pda,
        bump
    )]
    staking_pda_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        address=OHM_FORK_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    staking_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        address=ADMIN.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidOwner
    )]
    admin: Signer<'info>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    rent: Sysvar<'info, Rent>
}

pub fn ohm_initialize(ctx: Context<OhmInitialize>) -> Result<()>{
    Ok(())
}