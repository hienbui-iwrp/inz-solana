use anchor_lang::{
    prelude::*,
    solana_program::{keccak, program_error::ProgramError, secp256k1_recover::secp256k1_recover},
    system_program::{transfer as transfer_sol, Transfer as TransferSol},
};
use anchor_spl::token::{transfer, Transfer};

use context::*;
use error::*;
use event::*;
use state::*;

mod context;
mod error;
mod event;
mod state;
declare_id!("syX5i3s8c8aGXaESjcvv9xxhqVq267yG35HNtEGrxZ9");

#[program]
pub mod inz_mkp {
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
        system_config.signer = SystemConfig::DEFAULT_SIGNER;

        Ok(())
    }

    pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()> {
        // get data
        let mint = &ctx.accounts.mint;
        let from_token_account = &ctx.accounts.from_token_account;
        let to_token_account = &ctx.accounts.to_token_account;
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;
        let token_program = &ctx.accounts.token_program;

        // -- transfer nft --
        let transfer_cpi = CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: from_token_account.to_account_info(),
                to: to_token_account.to_account_info(),
                authority: from.to_account_info(),
            },
        );
        transfer(transfer_cpi, 1)?;

        emit!(TransferNftEvent {
            from: from.key(),
            to: to.key(),
            mint: mint.key(),
        });

        Ok(())
    }

    pub fn trade(
        ctx: Context<Trade>,
        price: u64,
        signature: [u8; 64],
        recovery_id: u8,
    ) -> Result<()> {
        // get data
        let mint = &ctx.accounts.mint;
        let from_token_account = &ctx.accounts.from_token_account;
        let seller = &ctx.accounts.seller;
        let to_token_account = &ctx.accounts.to_token_account;
        let buyer = &ctx.accounts.buyer;
        let fee_wallet = &ctx.accounts.fee_wallet;
        let system_config = &ctx.accounts.system_config;
        let token_program = &ctx.accounts.token_program;
        let system_program = &ctx.accounts.system_program;

        // verify
        let msg1 = mint.key().to_string();
        let msg2 = price.to_string();

        let message = format!("{msg1}{msg2}");
        msg!("message {}", message);

        let message_hash = {
            let mut hasher = keccak::Hasher::default();
            hasher.hash(message.as_bytes());
            hasher.result()
        };

        let recovered_pubkey = secp256k1_recover(&message_hash.0, recovery_id, &signature)
            .map_err(|_| ProgramError::InvalidArgument)?;

        // -- Check condition --
        if recovered_pubkey.0 != system_config.signer {
            return Err(error!(MkpError::WrongSignature));
        }

        // --- transfer price ---
        let transfer_price_context = CpiContext::new(
            system_program.to_account_info(),
            TransferSol {
                from: buyer.to_account_info(),
                to: seller.to_account_info(),
            },
        );
        transfer_sol(transfer_price_context, price)?;

        // -- transfer platform fee --
        let transfer_price_context = CpiContext::new(
            system_program.to_account_info(),
            TransferSol {
                from: buyer.to_account_info(),
                to: fee_wallet.to_account_info(),
            },
        );
        transfer_sol(transfer_price_context, system_config.platform_fee)?;

        // -- transfer nft --
        let signer_seeds: &[&[&[u8]]] = &[&[SystemConfig::SEED_PREFIX, &[ctx.bumps.system_config]]];
        let transfer_cpi = CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: from_token_account.to_account_info(),
                to: to_token_account.to_account_info(),
                authority: system_config.to_account_info(),
            },
        )
        .with_signer(signer_seeds);
        transfer(transfer_cpi, 1)?;

        emit!(TradeEvent {
            from: seller.key(),
            to: buyer.key(),
            mint: mint.key(),
            price: price,
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

    pub fn set_signer(ctx: Context<SetSigner>, signer: [u8; 64]) -> Result<()> {
        let config_account = &mut ctx.accounts.config_account;

        config_account.signer = signer;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
