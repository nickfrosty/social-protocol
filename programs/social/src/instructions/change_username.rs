use anchor_lang::prelude::*;

use crate::errors::GenericError;
use crate::state:: { Profile, LookupAccount } ;

#[derive(Accounts)]
#[instruction(profile_seed: [u8;32], new_username: String)]
pub struct ChangeUsername<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    /// the `profile.authority` that will be used to verify ownership
    pub authority: Signer<'info>,

    #[account(
        mut,
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
        space = LookupAccount::SPACE,
        seeds = [
            LookupAccount::PREFIX_SEED.as_ref(),
            Profile::PREFIX_SEED.as_ref(),
            // use the new username to derive the new lookup account
            new_username.as_ref()
        ],
        bump,
    )]
    pub new_lookup_account: Account<'info, LookupAccount>,

    #[account(
        mut,
        // when closing the old lookup account, send the lamports to the new lookup account
        // this makes changing usernames a negligible cost
        close = new_lookup_account,
        seeds = [
            LookupAccount::PREFIX_SEED.as_ref(),
            Profile::PREFIX_SEED.as_ref(),
            // use the current username to derive the old lookup account address
            profile.username.as_ref()
        ],
        bump = old_lookup_account.bump,
        // ensure the lookup account is owned by the profile's PDA
        constraint = old_lookup_account.authority.key() == profile.key() @ GenericError::Unauthorized,
    )]
    pub old_lookup_account: Account<'info, LookupAccount>,
}

///
pub fn process_change_username(ctx: Context<ChangeUsername>, _profile_seed: [u8; 32], new_username: String) -> Result<()> {
    Profile::validate_username(&new_username)?;

    // store the new lookup account's data 
    ctx.accounts.new_lookup_account.set_inner(LookupAccount {
        bump: ctx.bumps.new_lookup_account,
        // store the profile's address for easy retrieval by anyone
        address: ctx.accounts.profile.key(),
        // the profile PDA is set as the authority so that when the `profile.authority` changes, 
        // the same profile will still be able to update the inner data of this account
        authority: ctx.accounts.profile.key()
    });

    // actually update the username
    ctx.accounts.profile.username = new_username;

    // emit an event for indexers to observe
    // todo

    Ok(())
}