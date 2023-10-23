use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state:: { Profile, NameService } ;

#[derive(Accounts)]
#[instruction(profile_seed: [u8;32], new_username: String)]
pub struct ChangeUsername<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

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
            new_username.as_ref()
        ],
        bump,
    )]
    pub new_name_service: Account<'info, NameService>,

    #[account(
        mut,
        close = new_name_service,
        seeds = [
            NameService::PREFIX_SEED.as_ref(),
            Profile::PREFIX_SEED.as_ref(),
            profile.username.as_ref()
        ],
        bump = old_name_service.bump,
        // ensure the profile's authority is actually approving this
        has_one = authority @ GenericError::Unauthorized,
    )]
    pub old_name_service: Account<'info, NameService>,
}

///
pub fn process_change_username(ctx: Context<ChangeUsername>, _profile_seed: [u8; 32], new_username: String) -> Result<()> {
    Profile::validate_username(&new_username)?;

    // store the new name service's account data 
    ctx.accounts.new_name_service.set_inner(NameService {
        bump: ctx.bumps.new_name_service,
        address: ctx.accounts.profile.key(),
        authority: ctx.accounts.profile.authority.key()
    });

    // emit an event for indexers to observe
    // todo

    Ok(())
}