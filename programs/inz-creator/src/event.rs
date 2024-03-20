use anchor_lang::prelude::*;

use crate::NftType;

/// EVENT
/// * address: public key of new nft collection
/// * nft_types: list config type  
/// * owner: owner of nft collection
/// * name: name of nft
/// * symbol: symbol of nft
/// * uri: uri of nft
/// * callback_data: custom string that backend identify
#[event]
pub struct CreateCollectionEvent {
    pub address: Pubkey,
    pub nft_types: Vec<NftType>,
    pub owner: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub callback_data: String,
}

/// * address: public key of new nft
/// * name: name of nft
/// * symbol: symbol of nft
/// * uri: uri of nft
/// * collection: public key of nft collection that nft belongs
/// * owner: owner of nft
/// * nft_type: type of nft
#[event]
pub struct MintNftEvent {
    pub address: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub collection: Pubkey,
    pub owner: Pubkey,
    pub nft_type: u8,
}
