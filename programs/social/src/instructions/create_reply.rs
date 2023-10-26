use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{Post, Profile};

#[derive(Accounts)]
#[instruction(metadata_uri: String)]
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
            parent_post.group.key().as_ref(),
            parent_post.post_id.to_string().as_bytes(),
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
            // the parent post is used as the group for its child reply posts
            parent_post.key().as_ref(),
            // the current `reply_count` is intentionally used here
            parent_post.reply_count.to_string().as_bytes(),
        ],
        bump,
    )]
    pub reply: Account<'info, Post>,
}

/// Create a reply Post to an existing Post
pub fn process_create_reply(ctx: Context<CreateReply>, metadata_uri: String) -> Result<()> {
    // validate the input
    Post::validate_uri(&metadata_uri)?;

    // todo: ensure a parent post was actually provided since we are creating a reply
    // if no parent post was provided, this should error

    // actually store the provided data in the account
    ctx.accounts.reply.set_inner(Post {
        bump: ctx.bumps.reply,
        // even though a reply post's address is derived from the `parent_post`,
        // we still track the parent's group for easy access
        group: ctx.accounts.parent_post.group.key(),
        author: ctx.accounts.author.key(),
        metadata_uri: metadata_uri,
        parent_post: Some(ctx.accounts.parent_post.key()),
        reply_count: 0,
        /// reply post addresses are derived from the parent post's reply
        /// counter vice the post group's counter
        post_id: ctx.accounts.parent_post.reply_count,
    });

    /*
     * increment the parent post's reply counter
     *
     * note: we increment the post counter after the `reply.post_id` has been
     * updated to ensure we do not skip over any counts
     * e.g. the first reply's post_id should be 0 but if we increment it
     * before storing the post_id, it would be 1
     */
    ctx.accounts.parent_post.reply_count += 1;
    // todo: safe math

    // emit an event for indexers to observe
    // todo

    Ok(())
}
