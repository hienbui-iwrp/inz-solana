use anchor_lang::prelude::*;

#[error_code]
pub enum CreatorError {
    #[msg("Nft types are required")]
    TypesRequired,

    #[msg("Nfts are sold out!")]
    NftSoldOut,

    #[msg("Wrong collection owner!")]
    WrongCollectionOwner,

    #[msg("Caller is not admin!")]
    NotAdmin,

    #[msg("Wrong fee wallet!")]
    WrongFeeWallet,
}
