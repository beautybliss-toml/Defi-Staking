use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

use crate::{
    ADMIN, FWOG_MEME_COIN, BONK_MEME_COIN, POPCAT_MEME_COIN, REWARD_TOKEN_MINT_ADDRESS
};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct MemeCoinInitialize<'info>{
    /// CHECK
    #[account(
        init_if_needed,
        seeds=[b"meme_coin_pda"],
        payer=admin,
        bump,
        space=8
    )]
    meme_coin_pda: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds=[b"meme_coin_pda", meme_coin_mint.key().as_ref()],
        token::mint=meme_coin_mint,
        token::authority=meme_coin_pda,
        payer=admin,
        bump
    )]
    meme_coin_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=( meme_coin_mint.key() == FWOG_MEME_COIN.parse::<Pubkey>().unwrap() || 
            meme_coin_mint.key() == BONK_MEME_COIN.parse::<Pubkey>().unwrap() || 
            meme_coin_mint.key() == POPCAT_MEME_COIN.parse::<Pubkey>().unwrap() ) @ ErrorCode::NotAllowMemeCoin
    )]
    meme_coin_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=admin,
        seeds=[b"reward_token_pda"],
        space=8,
        bump
    )]
    reward_token_pda: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds=[b"reward_token_ata", REWARD_TOKEN_MINT_ADDRESS.as_bytes()],
        payer=admin,
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

    #[account(
        mut,
        address=ADMIN.parse::<Pubkey>().unwrap() @ ErrorCode::InvalidOwner
    )]
    admin: Signer<'info>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    rent: Sysvar<'info, Rent>
}

pub fn meme_coin_initialize(ctx: Context<MemeCoinInitialize>) -> Result<()> {
    Ok(())
}