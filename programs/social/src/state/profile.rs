use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::GenericError;

#[account]
#[derive(Default)]
pub struct Profile {
    /// bump used to derive the PDA
    pub bump: u8,

    /// random seed bytes used to derive the PDA for the Profile's account
    pub random_seed: [u8; 32],

    /// owner with blanket authority over the Profile
    pub authority: Pubkey,

    /// display name to be used for the Profile
    pub username: String,

    /// display name to be used for the Profile
    pub name: String,

    /// uri to an off-chain image to be displayed for the profile
    pub image_uri: String,

    /// uri to an off-chain JSON metadata file for additional profile information
    pub metadata_uri: String,
}

impl Profile {
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED : &str = "profile";

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>() + 
        // string name
        MAX_LEN_NAME + 
        // string `image_uri`
        MAX_LEN_URI + 
        // string `metadata_uri`
        MAX_LEN_URI;

    /// validate the standard generic input
    pub fn validate_input(input: &Profile) -> Result<()>{
        require!(input.name.len() <= MAX_LEN_NAME, GenericError::NameTooLong);
        require!(input.username.len() <= MAX_LEN_USERNAME, GenericError::NameTooLong);
        require!(input.metadata_uri.len() <= MAX_LEN_URI, GenericError::UriTooLong);
        require!(input.image_uri.len() <= MAX_LEN_URI, GenericError::UriTooLong);
    
        Self::validate_username(&input.username)?;

        Ok(())
    }
    
    /// validate the standard generic input
    pub fn validate_username(_username: &String) -> Result<()>{
    
        // todo: validate username for a set character set [a-z0-9_-]

        Ok(())
    }
}