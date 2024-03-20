use anchor_lang::prelude::*;

/// * from: address of nft owner
/// * to: address of buyer
/// * mint_address: address of nft
/// * price: price of listing nft
#[event]
pub struct TransferNftEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub mint: Pubkey,
}

/// * from: address of nft owner
/// * to: address of buyer
/// * mint_address: address of nft
/// * price: price of listing nft
#[event]
pub struct TradeEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
}
