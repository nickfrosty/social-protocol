use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state:: { Profile, NameService } ;

#[derive(Accounts)]
#[instruction(input: Profile)]
pub struct CreateProfile<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

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
            b"profile".as_ref(),
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
        address: ctx.accounts.profile.key(),
        authority: ctx.accounts.authority.key()
    });
    
    // store the provided input data into the account
    ctx.accounts.profile.set_inner(Profile {
        bump: ctx.bumps.profile,
        authority: *ctx.accounts.authority.key,
        random_seed: input.random_seed,
        name: input.name,
        username: input.username,
        metadata_uri : input.metadata_uri,
        image_uri: input.image_uri
    });

    // emit an event for indexers to observe
    // todo

    Ok(())
}


#[derive(Accounts)]
#[instruction(input: Profile)]
pub struct UpdateProfile<'info>{
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
    )]
    pub profile: Account<'info, Profile>,
}

pub fn process_update_profile(ctx: Context<UpdateProfile>, input: Profile) -> Result<()>{
    Profile::validate_input(&input)?;

    let profile = &mut ctx.accounts.profile;

    // perform security checks
    require_keys_eq!(profile.authority.key(), ctx.accounts.authority.key(), GenericError::Unauthorized);

    // update the desired profile details
    profile.name = input.name;
    profile.image_uri = input.image_uri;
    profile.metadata_uri= input.metadata_uri;

    // emit an event for indexers to observe
    // todo

    Ok(())
}

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
        has_one = authority @ GenericError::Unauthorized,
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        init, 
        payer = payer,
        space = NameService::SPACE,
        seeds = [
            NameService::PREFIX_SEED.as_ref(),
            b"profile".as_ref(),
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
            b"profile".as_ref(),
            profile.username.as_ref()
        ],
        bump = old_name_service.bump,
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


// todo: the following methods are yet to be implemented
// - change owner
// 