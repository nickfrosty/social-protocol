use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{Post, Profile};

#[derive(Accounts)]
#[instruction(random_seed: [u8; 32], metadata_uri: String)]
pub struct UpdatePost<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// the post author's authority
    pub authority: Signer<'info>,

    #[account(
        // ensure the author is actually approving this
        has_one = authority @ GenericError::Unauthorized
    )]
    pub author: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [
            Post::PREFIX_SEED.as_ref(),
            random_seed.as_ref()
        ],
        bump = post.bump,
        // ensure the provided author owns this Post
        has_one = author,
    )]
    pub post: Account<'info, Post>,
}

pub fn process_update_post(
    ctx: Context<UpdatePost>,
    _random_seed: [u8; 32],
    metadata_uri: String,
) -> Result<()> {
    Post::validate_uri(&metadata_uri)?;

    // only update the desired data
    ctx.accounts.post.metadata_uri = metadata_uri;

    // emit an event for indexers to observe
    // todo

    Ok(())
}
