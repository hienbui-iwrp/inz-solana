use anchor_lang::{
    prelude::*,
    system_program::{transfer as transfer_sol, Transfer as TransferSol},
};
use anchor_spl::{
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3,
    },
    token::{mint_to, MintTo},
};
use mpl_token_metadata::types::{Creator, DataV2};

use context::*;
use error::*;
use event::*;
use state::*;

mod context;
mod error;
mod event;
mod state;

declare_id!("9A4dT3GgZsnGYV62MzR48k9R1Y4TTJdTsxxxLAATsSvY");

#[program]
pub mod inz_creator {
    use super::*;

    /// Use for init new config account of marketplace
    /// platform_fee
    /// signer: signer of backend
    pub fn init_config(ctx: Context<InitConfig>, platform_fee: u64) -> Result<()> {
        let admin = &ctx.accounts.admin;
        let system_config = &mut ctx.accounts.system_config;
        let fee_wallet = &ctx.accounts.fee_wallet;

        system_config.platform_fee = platform_fee;
        system_config.fee_wallet = fee_wallet.key();
        system_config.admin = admin.key();

        Ok(())
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        nft_types: Vec<NftType>,
        name: String,
        symbol: String,
        uri: String,
        callback_data: String,
    ) -> Result<()> {
        //  -- Get account data --
        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        let owner = &ctx.accounts.owner;
        let metadata_account = &ctx.accounts.metadata_account;
        let master_edition_account = &ctx.accounts.master_edition_account;
        let config_account = &mut ctx.accounts.config_account;
        let token_program = &ctx.accounts.token_program;
        let token_metadata_program = &ctx.accounts.token_metadata_program;
        let system_program = &ctx.accounts.system_program;
        let rent = &ctx.accounts.rent;

        // -- Mint new nft --
        let mint_to_cpi_account = MintTo {
            mint: mint.to_account_info().clone(),
            to: token_account.to_account_info().clone(),
            authority: owner.to_account_info().clone(),
        };

        let mint_to_ctx =
            CpiContext::new(token_program.to_account_info().clone(), mint_to_cpi_account);
        mint_to(mint_to_ctx, 1)?;

        // -- Create metadata account --
        let cpi_context = CpiContext::new(
            token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: metadata_account.to_account_info(),
                mint: mint.to_account_info(),
                mint_authority: owner.to_account_info(),
                update_authority: owner.to_account_info(),
                payer: owner.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
        );

        let creators = vec![Creator {
            address: owner.key(),
            verified: false,
            share: 100,
        }];

        let data_v2 = DataV2 {
            name: name.clone(),
            symbol: symbol.clone(),
            uri: uri.clone(),
            seller_fee_basis_points: 0,
            creators: Some(creators.clone()),
            collection: None,
            uses: None,
        };
        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        //create master edition account
        let cpi_context = CpiContext::new(
            token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: master_edition_account.to_account_info(),
                mint: mint.to_account_info(),
                update_authority: owner.to_account_info(),
                mint_authority: owner.to_account_info(),
                payer: owner.to_account_info(),
                metadata: metadata_account.to_account_info(),
                token_program: token_program.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
        );
        create_master_edition_v3(cpi_context, None)?;

        // -- Set configuration --
        config_account.collection_key = mint.key();
        config_account.owner = owner.key();

        for nft_type in nft_types.iter() {
            if nft_type.supply == 0 {
                config_account.nft_types.push(NftType {
                    id: nft_type.id,
                    price: nft_type.price,
                    supply: u64::MAX,
                    minted: 0,
                })
            } else {
                config_account.nft_types.push(nft_type.clone());
            }
        }

        emit!(CreateCollectionEvent {
            address: mint.key(),
            name: name.clone(),
            symbol: symbol.clone(),
            uri: uri.clone(),
            owner: owner.key(),
            nft_types: nft_types.clone(),
            callback_data: callback_data.clone()
        });
        Ok(())
    }

    pub fn mint_nft(
        ctx: Context<MintNft>,
        name: String,
        symbol: String,
        uri: String,
        nft_type: u8,
    ) -> Result<()> {
        //  -- Get account data --
        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        let owner = &ctx.accounts.owner;
        let metadata_account = &ctx.accounts.metadata_account;
        let master_edition_account = &ctx.accounts.master_edition_account;
        let config_account = &mut ctx.accounts.config_account;
        let collection_owner = &ctx.accounts.collection_owner;
        let system_config = &ctx.accounts.system_config;
        let fee_wallet = &ctx.accounts.fee_wallet;
        let token_program = &ctx.accounts.token_program;
        let token_metadata_program = &ctx.accounts.token_metadata_program;
        let system_program = &ctx.accounts.system_program;
        let rent = &ctx.accounts.rent;

        let nft_types = &mut config_account.nft_types;

        // -- Get config of nft type --
        let mut nft_config = &mut NftType {
            id: 99,
            price: 0,
            supply: 0,
            minted: 0,
        };

        for config in nft_types.iter_mut() {
            if config.id == nft_type {
                nft_config = config;
            }
        }

        if nft_config.minted >= nft_config.supply {
            return Err(error!(CreatorError::NftSoldOut));
        }

        // update config
        nft_config.minted += 1;

        // -- Transfer price --
        let transfer_price_context = CpiContext::new(
            system_program.to_account_info(),
            TransferSol {
                from: owner.to_account_info(),
                to: collection_owner.to_account_info(),
            },
        );
        transfer_sol(transfer_price_context, nft_config.price)?;

        // -- Transfer platform fee --
        let transfer_fee_context = CpiContext::new(
            system_program.to_account_info(),
            TransferSol {
                from: owner.to_account_info(),
                to: fee_wallet.to_account_info(),
            },
        );
        transfer_sol(transfer_fee_context, system_config.platform_fee)?;

        // -- Mint new nft --
        let mint_to_cpi_account = MintTo {
            mint: mint.to_account_info().clone(),
            to: token_account.to_account_info().clone(),
            authority: owner.to_account_info().clone(),
        };

        let mint_to_ctx =
            CpiContext::new(token_program.to_account_info().clone(), mint_to_cpi_account);
        mint_to(mint_to_ctx, 1)?;

        // -- Create metadata account --
        let cpi_context = CpiContext::new(
            token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: metadata_account.to_account_info(),
                mint: mint.to_account_info(),
                mint_authority: owner.to_account_info(),
                update_authority: owner.to_account_info(),
                payer: owner.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
        );

        let creators = vec![Creator {
            address: owner.key(),
            verified: false,
            share: 100,
        }];

        let data_v2 = DataV2 {
            name: name.clone(),
            symbol: symbol.clone(),
            uri: uri.clone(),
            seller_fee_basis_points: 0,
            creators: Some(creators.clone()),
            collection: None,
            uses: None,
        };
        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        //create master edition account
        let cpi_context = CpiContext::new(
            token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: master_edition_account.to_account_info(),
                mint: mint.to_account_info(),
                update_authority: owner.to_account_info(),
                mint_authority: owner.to_account_info(),
                payer: owner.to_account_info(),
                metadata: metadata_account.to_account_info(),
                token_program: token_program.to_account_info(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
        );
        create_master_edition_v3(cpi_context, None)?;

        // emit log
        emit!(MintNftEvent {
            address: mint.key(),
            name: name,
            symbol: symbol,
            uri: uri,
            collection: config_account.collection_key.clone(),
            owner: owner.key(),
            nft_type: nft_type,
        });

        Ok(())
    }

    pub fn set_admin(ctx: Context<SetAdmin>) -> Result<()> {
        let new_admin = &ctx.accounts.new_admin;
        let config_account = &mut ctx.accounts.config_account;

        config_account.admin = new_admin.key();

        Ok(())
    }

    pub fn set_fee_wallet(ctx: Context<SetFeeWallet>) -> Result<()> {
        let config_account = &mut ctx.accounts.config_account;
        let fee_wallet = &ctx.accounts.fee_wallet;

        config_account.fee_wallet = fee_wallet.key();

        Ok(())
    }

    pub fn set_platform_fee(ctx: Context<SetPlatformFee>, platform_fee: u64) -> Result<()> {
        let config_account = &mut ctx.accounts.config_account;

        config_account.platform_fee = platform_fee;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
