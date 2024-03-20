use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{ Mint, Token, TokenAccount },
};
use std::str::FromStr;
use crate::{ CollectionConfig, NftType, SystemConfig, CreatorError };

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(
        mut, 
        address = Pubkey::from_str(SystemConfig::DEFAULT_ADMIN).unwrap() @ CreatorError::NotAdmin
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
#[instruction(nft_types: Vec<NftType>)]
pub struct CreateCollection<'info> {
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = owner.key(),
        mint::freeze_authority = owner.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: From metaplex
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: From metaplex
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,
    #[account(
        init,
        payer = owner,
        space = CollectionConfig::MIN_SIZE + nft_types.len() * NftType::MIN_SIZE,
        seeds = [CollectionConfig::SEED_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub config_account: Account<'info, CollectionConfig>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = owner.key(),
        mint::freeze_authority = owner.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: From metaplex
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: From metaplex
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,
    /// CHECK
    #[account(mut, address = config_account.owner @ CreatorError::WrongCollectionOwner )]
    pub collection_owner: AccountInfo<'info>,
    #[account(mut)]
    pub config_account: Account<'info, CollectionConfig>,
    #[account(
        mut,
        seeds = [SystemConfig::SEED_PREFIX],
        bump
    )]
    pub system_config: Account<'info, SystemConfig>,
    /// CHECK
    #[account(mut, address = system_config.fee_wallet @ CreatorError::WrongFeeWallet)]
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

/// * admin: admin of marketplace
/// * config_account: config_account save config
/// * new_admin: new admin account of system
/// * system_program: system program
#[derive(Accounts)]
pub struct SetAdmin<'info> {
    #[account(mut, address = config_account.admin @ CreatorError::NotAdmin)]
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
    #[account(mut, address = config_account.admin @ CreatorError::NotAdmin)]
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
    #[account(mut, address = config_account.admin @ CreatorError::NotAdmin)]
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
