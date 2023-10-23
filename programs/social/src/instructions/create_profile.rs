use anchor_lang::prelude::*;
use crate::state:: { Profile, NameService } ;

#[derive(Accounts)]
#[instruction(input: Profile)]
pub struct CreateProfile<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    /// the authority to be set for this Profile
    pub authority: Signer<'info>,

    #[account(
        init, 
        payer = payer,
        space = Profile::SPACE,
        seeds = [
            Profile::PREFIX_SEED.as_ref(),
            input.random_seed.as_ref()
        ],
        bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        init, 
        payer = payer,
        space = NameService::SPACE,
        seeds = [
            NameService::PREFIX_SEED.as_ref(),
            Profile::PREFIX_SEED.as_ref(),
            input.username.as_ref()
        ],
        bump
    )]
    pub name_service: Account<'info, NameService>,
}

///
pub fn process_create_profile(ctx: Context<CreateProfile>, input: Profile) -> Result<()> {
    Profile::validate_input(&input)?;
    
    // store the new name service's account data 
    ctx.accounts.name_service.set_inner(NameService { 
        bump: ctx.bumps.name_service,
        // store the profile's address for easy retrieval by anyone
        address: ctx.accounts.profile.key(),
        // the profile PDA is set as the authority so that when the `profile.authority` changes, 
        // the same profile will still be able to update the inner data of this account
        authority: ctx.accounts.profile.key()
    });
    
    // store the provided input data into the account
    ctx.accounts.profile.set_inner(Profile {
        bump: ctx.bumps.profile,
        random_seed: input.random_seed,
        name: input.name,
        username: input.username,
        metadata_uri : input.metadata_uri,
        image_uri: input.image_uri,
        // set the profile's authority to be the provided `authority` 
        // since it is already a signer on the transaction
        authority: ctx.accounts.authority.key(),
    });

    // emit an event for indexers to observe
    // todo

    Ok(())
}
