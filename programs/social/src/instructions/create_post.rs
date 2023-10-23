use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{Post, Profile};

#[derive(Accounts)]
#[instruction(random_seed: [u8; 32], metadata_uri: String)]
pub struct CreatePost<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// the author's authority
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            author.random_seed.as_ref()
        ],
        bump = author.bump,
        
        // ensure the post author is actually approving this
        has_one = authority @ GenericError::Unauthorized
    )]
    pub author: Account<'info, Profile>,

    #[account(
        init,
        payer=payer,
        space=Post::SPACE,
        seeds = [
            Post::PREFIX_SEED.as_ref(),
            random_seed.as_ref()
        ],
        bump,
    )]
    pub post: Account<'info, Post>,
}

/// Create a root Post that is published by the `author` (aka `Profile`)
pub fn process_create_post(
    ctx: Context<CreatePost>,
    random_seed: [u8; 32],
    metadata_uri: String,
) -> Result<()> {
    // validate the input
    Post::validate_uri(&metadata_uri)?;

    // actually store the provided data in the account
    ctx.accounts.post.set_inner(Post {
        bump: ctx.bumps.post,
        random_seed,
        metadata_uri,
        author: ctx.accounts.author.key(),
        // no replies to start :)
        reply_count: 0,
        // parent post is set to None when creating root a Post
        parent_post: None,
    });

    // emit an event for indexers to observe
    // todo

    Ok(())
}
