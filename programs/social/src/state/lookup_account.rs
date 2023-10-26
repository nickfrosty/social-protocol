use anchor_lang::prelude::*;

#[account]
pub struct LookupAccount {
    /// bump used to derive the PDA
    pub bump: u8,

    /// account address this lookup account points too
    pub address: Pubkey,

    /// the account that has the ability to change the the details of the `LookupAccount`
    pub authority: Pubkey,
}

impl LookupAccount {
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED: &str = "lookup";

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>();
}
