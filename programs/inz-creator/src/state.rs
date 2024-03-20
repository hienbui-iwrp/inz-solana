use anchor_lang::prelude::*;

#[account]
pub struct SystemConfig {
    pub platform_fee: u64,
    pub fee_wallet: Pubkey,
    pub admin: Pubkey,
    pub bump: u8,
}

impl SystemConfig {
    pub const MIN_SIZE: usize = 8 +
        8 + // u64
        32 + // pubkey
        32 + // pubkey
        1;
    pub const SEED_PREFIX: &'static [u8; 6] = b"system";
    pub const DEFAULT_ADMIN: &str = "ABDxeRd7vcGVsTBCxA791fWWTXA8tQcXWWGrLcr9YvMS";
}

/// DATA ACCOUNT
/// * nft_types: List type of config
/// * collection_key: Public key of  nft collection
/// * owner: owner of nft collection
/// * bump: bump of PDA
#[account]
pub struct CollectionConfig {
    pub nft_types: Vec<NftType>,
    pub collection_key: Pubkey,
    pub owner: Pubkey,
    pub bump: u8,
}

impl CollectionConfig {
    pub const MIN_SIZE: usize = 8 +
        8 + // vec
        32 + // pubkey
        32 + // pubkey
        1;
    pub const SEED_PREFIX: &'static [u8; 6] = b"config";
}

/// * id: if of type
/// * price: price of each nft have this type
/// * supply: supply of this type
/// * minted: current minted nft in thist type
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct NftType {
    pub id: u8,
    pub price: u64,
    pub supply: u64,
    pub minted: u64,
}

impl NftType {
    pub const MIN_SIZE: usize = 1 + 8 + 8 + 8 + 8;
}
