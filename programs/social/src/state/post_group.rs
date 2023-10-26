use anchor_lang::prelude::*;

use crate::errors::GenericError;

#[account]
#[derive(Default)]
pub struct PostGroup {
    /// bump used to derive the PDA
    pub bump: u8,

    /// account with authority over the PostGroup
    pub authority: Pubkey,

    /// counter for total number of posts within the PostGroup.
    /// note: the `post_count` will be used as a seed for child Posts.
    /// effecively allowing us to easily enumerate child posts
    pub post_count: u32,
    
    /// simple string used to locate the PostGroup via a friendly name
    pub name: String,
}

impl PostGroup {
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED: &str = "post_group";

    /// max allowed length of the friendly name
    pub const MAX_LEN_NAME : usize = 32;

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>() + 
        // string `name`
        Self::MAX_LEN_NAME;

    /// validate the friendly `name` of a PostGroup
    pub fn validate_name(name: &String) -> Result<()>{
        require!(name.len() <= Self::MAX_LEN_NAME, GenericError::NameTooLong);

        Ok(())
    }
}
