use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::GenericError;

#[account]
#[derive(Default)]
pub struct Post {
    /// bump used to derive the PDA
    pub bump: u8,

    /// random seed bytes used to derive the PDA for the Post's account
    pub random_seed: [u8; 32],

    /// address of the PostGroup this post was published to
    pub group: Pubkey,

    /// author profile of the originating post
    pub author: Pubkey,

    /// parent post used to denote if a given post has been
    pub parent_post: Option<Pubkey>,

    /// tracks the total number of replies a give post has recieved.
    /// note: the `reply_count` will be used as a seed for child Posts.
    /// effecively allowing us to easily enumerate replies/child posts
    pub reply_count: u32,

    /// uri to an off-chain JSON metadata file that stores the actual post information
    pub metadata_uri: String,

}


impl Post {
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED : &str = "post";

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>() + 
        // string `metadata_uri`
        MAX_LEN_URI;

    ///
    pub fn validate_uri(uri: &String) -> Result<()> {
        require!(uri.len() <= MAX_LEN_URI, GenericError::UriTooLong);
        Ok(())
    }
}