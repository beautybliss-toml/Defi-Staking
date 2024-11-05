use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

use crate::errors::ErrorCode;
use crate::{OHM_FORK_MINT_ADDRESS, instructions::UserInfo};

#[derive(Accounts)]
pub struct ClaimOhm<'info>{
    /// CHECK
    #[account(
        mut,
        seeds=[b"staking_pda"],
        bump
    )]
    staking_pda: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[b"staking_pda_account", OHM_FORK_MINT_ADDRESS.as_bytes()],
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

    #[account(mut)]
    user_info: Account<'info, UserInfo>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(
        mut,
        constraint=user_ata.mint==*(staking_token_mint.to_account_info().key) && user_ata.owner==*(user.to_account_info().key)
    )]
    user_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    rent: Sysvar<'info, Rent>
}

pub fn claim_ohm(ctx: Context<ClaimOhm>, amount: u64) -> Result<()>{
    require!(amount > 0, ErrorCode::WrongAmount);

    let user_info = &mut ctx.accounts.user_info;
    require!(user_info.amount > 0, ErrorCode::NotEnough);

    let clock = Clock::get()?;
    let reward = &mut 0;

    *reward = user_info.amount / 10 * ( ( clock.slot - user_info.deposit_slot ) / ( 24 * 3600) );

    require!( ( user_info.reward + *reward ) >= amount, ErrorCode::NotEnough );

    let cpi_accounts = Transfer {
        from: ctx.accounts.staking_pda_account.to_account_info(),
        to: ctx.accounts.user_ata.to_account_info(),
        authority: ctx.accounts.staking_pda.to_account_info()
    };

    let bump = ctx.bumps.staking_pda;
    let seeds = &[b"staking_pda".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer
    );

    token::transfer(cpi_ctx, amount);

    user_info.deposit_slot = clock.slot;

    user_info.reward = user_info.reward + *reward - amount;

    Ok(())
}