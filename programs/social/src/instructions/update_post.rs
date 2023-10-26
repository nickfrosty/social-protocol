use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{Post, Profile};

#[derive(Accounts)]
#[instruction(metadata_uri: String)]
pub struct UpdatePost<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// the post author's authority
    pub authority: Signer<'info>,

    /// CHECK: the group can either be a `PostGroup` for "root posts"
    /// or a the parent `Post` for "reply posts"
    // todo: add some constraint checks to verify the post is a reply or root?
    #[account()]
    pub group: UncheckedAccount<'info>,
    #[account(
        // ensure the author is actually approving this
        has_one = authority @ GenericError::Unauthorized
    )]
    pub author: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [
            Post::PREFIX_SEED.as_ref(),
            // this group address can be either a PostGroup or a parent Post
            group.key().as_ref(),
            post.post_id.to_string().as_bytes(),
        ],
        bump = post.bump,
        // ensure the provided author owns this Post
        has_one = author,
    )]
    pub post: Account<'info, Post>,
}

pub fn process_update_post(ctx: Context<UpdatePost>, metadata_uri: String) -> Result<()> {
    Post::validate_uri(&metadata_uri)?;

    // only update the desired data
    ctx.accounts.post.metadata_uri = metadata_uri;

    // emit an event for indexers to observe
    // todo

    Ok(())
}
