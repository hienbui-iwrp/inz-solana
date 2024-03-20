use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use std::str::FromStr;

use crate::{ SystemConfig, MkpError};

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(
        mut, 
        address = Pubkey::from_str(SystemConfig::DEFAULT_ADMIN).unwrap() @ MkpError::NotAdmin
    )]
    pub admin: Signer<'info>,
    /// CHECK: Our wallet
    #[account(
        init,
        seeds = [SystemConfig::SEED_PREFIX],
        payer = admin,
        space = SystemConfig::MIN_SIZE,
        bump
    )]
    pub system_config: Account<'info, SystemConfig>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct TransferNft<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = mint,
        associated_token::authority = to
    )]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Trade<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub to_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    /// CHECK
    #[account(mut, address = system_config.fee_wallet @ MkpError::WrongFeeWallet)]
    pub fee_wallet: AccountInfo<'info>,
    /// CHECK: Our wallet
    #[account(
        init_if_needed,
        payer = buyer,
        space = SystemConfig::MIN_SIZE,
        seeds = [SystemConfig::SEED_PREFIX],
        bump
    )]
    pub system_config: Account<'info, SystemConfig>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}



/// * admin: admin of marketplace
/// * config_account: config_account save config
/// * new_admin: new admin account of system
/// * system_program: system program
#[derive(Accounts)]
pub struct SetAdmin<'info> {
    #[account(mut, address = config_account.admin @ MkpError::NotAdmin)]
    pub admin: Signer<'info>,
    /// CHECK: Our wallet
    #[account(
        mut,
        seeds = [SystemConfig::SEED_PREFIX],
        bump
    )]
    pub config_account: Account<'info, SystemConfig>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub new_admin: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

/// * admin: admin of marketplace
/// * config_account: config_account save config
/// * fee_wallet: new fee wallet account of system
/// * system_program: system program
#[derive(Accounts)]
pub struct SetFeeWallet<'info> {
    #[account(mut, address = config_account.admin @ MkpError::NotAdmin)]
    pub admin: Signer<'info>,
    /// CHECK: Our wallet
    #[account(
        mut,
        seeds = [SystemConfig::SEED_PREFIX],
        bump
    )]
    pub config_account: Account<'info, SystemConfig>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

/// * admin: admin of marketplace
/// * config_account: config_account save config
/// * system_program: system program
#[derive(Accounts)]
pub struct SetPlatformFee<'info> {
    #[account(mut, address = config_account.admin @ MkpError::NotAdmin)]
    pub admin: Signer<'info>,
    /// CHECK: Our wallet
    #[account(
        mut,
        seeds = [SystemConfig::SEED_PREFIX],
        bump
    )]
    pub config_account: Account<'info, SystemConfig>,
    pub system_program: Program<'info, System>,
}


/// * admin: admin of marketplace
/// * config_account: config_account save config
/// * system_program: system program
#[derive(Accounts)]
pub struct SetSigner<'info> {
    #[account(mut, address = config_account.admin @ MkpError::NotAdmin)]
    pub admin: Signer<'info>,
    /// CHECK: Our wallet
    #[account(
        mut,
        seeds = [SystemConfig::SEED_PREFIX],
        bump
    )]
    pub config_account: Account<'info, SystemConfig>,
    pub system_program: Program<'info, System>,
}

