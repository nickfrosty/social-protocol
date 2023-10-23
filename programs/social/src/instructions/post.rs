use anchor_lang::prelude::*;

use crate::errors::GenericError;
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
        bump
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

#[derive(Accounts)]
#[instruction(random_seed: [u8; 32], metadata_uri: String)]
pub struct CreateReply<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            author.random_seed.as_ref()
        ],
        bump
    )]
    pub author: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [
            Post::PREFIX_SEED.as_ref(),
            parent_post.random_seed.as_ref()
        ],
        bump,
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
    // todo

    // perform the security checks
    // todo

    let reply = &mut ctx.accounts.reply;

    // store the provided data in the account
    reply.set_inner(Post {
        random_seed,
        metadata_uri: metadata_uri,
        author: ctx.accounts.author.key(),
        parent_post: Some(ctx.accounts.parent_post.key()),
        reply_count: 0,
    });

    // increment the parent post's `reply_counter`
    ctx.accounts.parent_post.reply_count += 1;

    // emit an event for indexers to observe
    // todo

    Ok(())
}

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
        bump,
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
        ctx.accounts.author.owner.key(),
        ctx.accounts.authority.key(),
        GenericError::Unauthorized
    );

    // only update the desired data
    post.metadata_uri = metadata_uri;

    // emit an event for indexers to observe
    // todo

    Ok(())
}
