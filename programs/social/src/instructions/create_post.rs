use anchor_lang::prelude::*;

// use crate::errors::GenericError;
use crate::state::{Post, Profile};

#[derive(Accounts)]
#[instruction(random_seed: [u8; 32], metadata_uri: String)]
pub struct CreatePost<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            author.random_seed.as_ref()
        ],
        bump = author.bump,
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
    // todo

    // perform the security checks
    // todo

    let post = &mut ctx.accounts.post;

    // store the provided data in the account
    post.set_inner(Post {
        bump: ctx.bumps.post,
        random_seed,
        metadata_uri,
        author: ctx.accounts.author.key(),
        // note: these should always be set to empty values when creating a root Post
        parent_post: None,
        reply_count: 0,
    });

    // emit an event for indexers to observe
    // todo

    Ok(())
}
