use anchor_lang::prelude::*;
use anchor_lang::system_program;

use anchor_spl::token::{self, Token, Mint, Transfer, TokenAccount};

use crate::errors::ErrorCode;
use crate::{
    REWARD_TOKEN_MINT_ADDRESS,
    MemeStakingUserInfo
};

#[derive(Accounts)]
pub struct ClaimRewardForMemeCoin<'info>{
    #[account(
        mut,
        seeds=[b"reward_token_pda"],
        bump
    )]
    reward_token_pda: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"reward_token_ata", reward_token_mint.key().as_ref()],
        token::mint=reward_token_mint,
        token::authority=reward_token_pda,
        bump
    )]
    reward_token_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        address=REWARD_TOKEN_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    reward_token_mint: Account<'info, Mint>,

    #[account(mut)]
    meme_staking_user_info: Account<'info, MemeStakingUserInfo>,

    #[account(
        mut,
        constraint=user_reward_ata.mint==*(reward_token_mint.to_account_info().key) && user_reward_ata.owner==*(user.to_account_info().key)
    )]
    user_reward_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    rent: Sysvar<'info, Rent>

}

pub fn claim_reward_for_meme_coin(ctx: Context<ClaimRewardForMemeCoin>, amount: u64) -> Result<()> {
    // check amount
    require!(amount > 0, ErrorCode::WrongAmount);

    // calculate reward
    let meme_staking_user_info = &mut ctx.accounts.meme_staking_user_info;

    let clock = Clock::get()?;
    let reward = &mut 0;

    if meme_staking_user_info.amount > 0 {
        let slot = ( clock.slot - meme_staking_user_info.deposit_slot ) / ( 24 * 3600 );
        *reward = meme_staking_user_info.amount / 10 * slot; // adjust
    }

    require!( ( meme_staking_user_info.amount + *reward ) > amount, ErrorCode::NotEnough);

    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_token_ata.to_account_info(),
        to: ctx.accounts.user_reward_ata.to_account_info(),
        authority: ctx.accounts.reward_token_pda.to_account_info()
    };

    let bump = ctx.bumps.reward_token_pda;
    let seeds = &[b"reward_token_pda".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer
    );

    token::transfer(cpi_ctx, amount);

    meme_staking_user_info.deposit_slot = clock.slot;

    meme_staking_user_info.reward = *reward - amount;

    Ok(())
}