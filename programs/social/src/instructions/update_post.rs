use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{Post, Profile};

#[derive(Accounts)]
#[instruction(random_seed: [u8; 32], metadata_uri: String)]
pub struct UpdatePost<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub authority: Signer<'info>,

    #[account()]
    pub author: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [
            Post::PREFIX_SEED.as_ref(),
            random_seed.as_ref()
        ],
        bump = post.bump,
    )]
    pub post: Account<'info, Post>,
}

pub fn process_update_post(
    ctx: Context<UpdatePost>,
    _random_seed: [u8; 32],
    metadata_uri: String,
) -> Result<()> {
    Post::validate_uri(&metadata_uri)?;

    // perform the security checks
    // todo: ensure the `parent_post` is owned by our program
    // todo: ensure the `author` is owned by our program? should they be a `Profile`?

    let post = &mut ctx.accounts.post;

    // ensure the correct `author` was provided
    require_keys_eq!(
        post.author.key(),
        ctx.accounts.author.key(),
        GenericError::InvalidAccount
    );

    // only allow the owner of the author's profile to update the post
    require_keys_eq!(
        ctx.accounts.author.authority.key(),
        ctx.accounts.authority.key(),
        GenericError::Unauthorized
    );

    // only update the desired data
    post.metadata_uri = metadata_uri;

    // emit an event for indexers to observe
    // todo

    Ok(())
}
