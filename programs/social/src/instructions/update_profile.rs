use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state::Profile;

#[derive(Accounts)]
#[instruction(input: Profile)]
pub struct UpdateProfile<'info> {
    system_program: Program<'info, System>,

    #[account(mut)]
    payer: Signer<'info>,

    // #[account(mut)]
    authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            input.random_seed.as_ref()
        ],
        bump = profile.bump,
        has_one = authority @ GenericError::Unauthorized,
    )]
    pub profile: Account<'info, Profile>,
}

pub fn process_update_profile(ctx: Context<UpdateProfile>, input: Profile) -> Result<()> {
    Profile::validate_input(&input)?;

    let profile = &mut ctx.accounts.profile;

    // update the desired profile details
    profile.name = input.name;
    profile.image_uri = input.image_uri;
    profile.metadata_uri = input.metadata_uri;

    // emit an event for indexers to observe
    // todo

    Ok(())
}
