use anchor_lang::prelude::*;

#[account]
pub struct NameService {
    /// account address this name service account points too
    pub address: Pubkey,

    /// the account that has the ability to change the the details of this NameService
    pub authority: Pubkey,
    // pub seeds;
}

impl NameService {
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED: &str = "name_service";

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>();
}
