use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state:: { Profile, NameService } ;

#[derive(Accounts)]
#[instruction(profile_seed: [u8;32], new_username: String)]
pub struct ChangeUsername<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    /// the `profile.authority` that will be used to verify ownership
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            profile_seed.as_ref()
        ],
        bump = profile.bump,
        // ensure the profile's authority is actually approving this
        has_one = authority @ GenericError::Unauthorized,
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        init, 
        payer = payer,
        space = NameService::SPACE,
        seeds = [
            NameService::PREFIX_SEED.as_ref(),
            Profile::PREFIX_SEED.as_ref(),
            // use the new username to derive the new name service account
            new_username.as_ref()
        ],
        bump,
    )]
    pub new_name_service: Account<'info, NameService>,

    #[account(
        mut,
        // when closing the old name service, send the lamports to the new name service
        // this makes changing usernames a negligible cost
        close = new_name_service,
        seeds = [
            NameService::PREFIX_SEED.as_ref(),
            Profile::PREFIX_SEED.as_ref(),
            // use the current username to derive the old name service address
            profile.username.as_ref()
        ],
        bump = old_name_service.bump,
        // ensure the name service is owned by the profile's PDA
        constraint = old_name_service.authority.key() == profile.key() @ GenericError::Unauthorized,
    )]
    pub old_name_service: Account<'info, NameService>,
}

///
pub fn process_change_username(ctx: Context<ChangeUsername>, _profile_seed: [u8; 32], new_username: String) -> Result<()> {
    Profile::validate_username(&new_username)?;

    // store the new name service's account data 
    ctx.accounts.new_name_service.set_inner(NameService {
        bump: ctx.bumps.new_name_service,
        // store the profile's address for easy retrieval by anyone
        address: ctx.accounts.profile.key(),
        // the profile PDA is set as the authority so that when the `profile.authority` changes, 
        // the same profile will still be able to update the inner data of this account
        authority: ctx.accounts.profile.key()
    });

    // emit an event for indexers to observe
    // todo

    Ok(())
}