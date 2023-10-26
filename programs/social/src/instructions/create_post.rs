use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{Post, PostGroup, Profile};

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
            group.key().as_ref(),
            // we are intentionally using the current `post_count` instead of the next value
            group.post_count.to_string().as_bytes(),
        ],
        bump,
    )]
    pub post: Account<'info, Post>,

    #[account(
        mut,
        seeds = [
            PostGroup::PREFIX_SEED.as_ref(),
            group.random_seed.as_ref()
        ],
        bump,
        // ensure the post group is owned by the author PDA
        constraint = group.authority.key() == author.key() @ GenericError::Unauthorized,
    )]
    pub group: Account<'info, PostGroup>,
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
        group: ctx.accounts.group.key(),
        // we are intentionally using the current `post_count` vice the next value
        // this ensures we do not skip any index numbers
        // this works well because of the write lock feature of Solana :)
        post_id: ctx.accounts.group.post_count,
        author: ctx.accounts.author.key(),
        metadata_uri,
        // no replies to start :)
        reply_count: 0,
        // parent post is set to None when creating a "root post"
        parent_post: None,
    });

    // the `post_id` is effectively an "auto increment" value from the `group.post_count`
    // now that the current post count has been stored in the account as the `post_id`, 
    // we can auto increment the `group.post_count` for the next post to use
    ctx.accounts.group.post_count += 1;
    // todo: safe math

    // emit an event for indexers to observe
    // todo

    Ok(())
}
