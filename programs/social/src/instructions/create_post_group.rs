use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::{PostGroup, Profile, NameService};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreatePostGroup<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// the `author.authority` that will be used to verify ownership
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            author.random_seed.as_ref()
        ],
        bump = author.bump,
        
        // ensure the author's authority is actually approving this
        has_one = authority @ GenericError::Unauthorized
    )]
    pub author: Account<'info, Profile>,

    #[account(
        init,
        payer=payer,
        space=PostGroup::SPACE,
        seeds = [
            PostGroup::PREFIX_SEED.as_ref(),
            name.as_ref()
        ],
        bump,
    )]
    pub group: Account<'info, PostGroup>,

    #[account(
        init,
        payer = payer,
        space = NameService::SPACE,
        seeds = [
            NameService::PREFIX_SEED.as_ref(),
            PostGroup::PREFIX_SEED.as_ref(),
            name.as_ref()
        ],
        bump
    )]
    pub name_service: Account<'info, NameService>
}

/// Create a PostGroup that is published by the `author` (aka `Profile`)
pub fn process_create_post_group(
    ctx: Context<CreatePostGroup>,
    name: String
) -> Result<()> {
    // validate the input
    PostGroup::validate_name(&name)?;

    // create the name service account for the group being created
    ctx.accounts.name_service.set_inner(NameService { 
        bump: ctx.bumps.name_service,
        // store the group's address for easy retrieval by anyone
        address: ctx.accounts.group.key(),
        // the author PDA is set as the authority so that when the `author.authority` changes, 
        // the same author will still be able to update the inner data of this account
        authority: ctx.accounts.author.key(),
    });
    
    // actually store the provided data in the account
    ctx.accounts.group.set_inner(PostGroup {
        bump: ctx.bumps.group,
        name: name,
        post_count: 0,
        // the author PDA is set as the authority so that when the `author.authority` changes, 
        // the same author will still be able to update the inner data of this account
        authority: ctx.accounts.author.key(),
    });

    // emit an event for indexers to observe
    // todo

    Ok(())
}
