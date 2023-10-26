use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{Post, Profile};

#[derive(Accounts)]
#[instruction(random_seed: [u8; 32], metadata_uri: String)]
pub struct CreateReply<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// the reply author's authority
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            author.random_seed.as_ref()
        ],
        bump = author.bump,

        // ensure the reply author is actually approving this
        has_one = authority @ GenericError::Unauthorized
    )]
    pub author: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [
            Post::PREFIX_SEED.as_ref(),
            parent_post.random_seed.as_ref()
        ],
        bump = parent_post.bump,
    )]
    pub parent_post: Account<'info, Post>,

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
    pub reply: Account<'info, Post>,
}

/// Create a reply Post to an existing Post
pub fn process_create_reply(
    ctx: Context<CreateReply>,
    random_seed: [u8; 32],
    metadata_uri: String,
) -> Result<()> {
    // validate the input
    Post::validate_uri(&metadata_uri)?;

    // todo: ensure a parent post was actually provided since we are creating a reply
    // if no parent post was provided, this should error

    // actually store the provided data in the account
    ctx.accounts.reply.set_inner(Post {
        bump: ctx.bumps.reply,
        random_seed,
        group: ctx.accounts.parent_post.group.key(),
        author: ctx.accounts.author.key(),
        metadata_uri: metadata_uri,
        parent_post: Some(ctx.accounts.parent_post.key()),
        reply_count: 0,
    });

    // increment the parent post's `reply_counter`
    ctx.accounts.parent_post.reply_count += 1;
    // todo: safe math

    // emit an event for indexers to observe
    // todo

    Ok(())
}
