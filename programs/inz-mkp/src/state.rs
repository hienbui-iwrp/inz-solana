use anchor_lang::prelude::*;

#[account]
pub struct SystemConfig {
    pub platform_fee: u64,
    pub fee_wallet: Pubkey,
    pub admin: Pubkey,
    pub signer: [u8; 64],
    pub bump: u8,
}

impl SystemConfig {
    pub const MIN_SIZE: usize = 8 +
        8 + // u64
        32 + // pubkey
        32 + // pubkey
        64 + // array
        1;
    pub const SEED_PREFIX: &'static [u8; 6] = b"system";
    pub const DEFAULT_ADMIN: &str = "ABDxeRd7vcGVsTBCxA791fWWTXA8tQcXWWGrLcr9YvMS";
    pub const DEFAULT_SIGNER: [u8; 64] = [
        226, 207, 81, 133, 190, 78, 152, 231, 42, 121, 162, 247, 3, 148, 28, 161, 194, 166, 249,
        26, 238, 211, 85, 208, 177, 154, 232, 124, 205, 112, 177, 54, 206, 77, 141, 226, 246, 96,
        7, 199, 142, 62, 244, 147, 254, 117, 28, 42, 253, 62, 51, 142, 3, 98, 196, 69, 75, 145, 89,
        162, 1, 80, 89, 221,
    ];
}
