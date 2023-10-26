use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::*;
use crate::state::*;

declare_id!("EsNiAoa8UtvZ81e1um5KLmW79xTq8DzyKvX4nZAG9TL6");

#[program]
pub mod social {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, input: Profile) -> Result<()> {
        process_create_profile(ctx, input)
    }
    pub fn update_profile(ctx: Context<UpdateProfile>, input: Profile) -> Result<()> {
        process_update_profile(ctx, input)
    }
    pub fn change_username(
        ctx: Context<ChangeUsername>,
        random_seed: [u8; 32],
        new_username: String,
    ) -> Result<()> {
        process_change_username(ctx, random_seed, new_username)
    }
    pub fn create_post(
        ctx: Context<CreatePost>,
        random_seed: [u8; 32],
        metadata_uri: String,
    ) -> Result<()> {
        process_create_post(ctx, random_seed, metadata_uri)
    }
    pub fn create_post_group(ctx: Context<CreatePostGroup>, name: String) -> Result<()> {
        process_create_post_group(ctx, name)
    }
    pub fn create_reply(
        ctx: Context<CreateReply>,
        random_seed: [u8; 32],
        metadata_uri: String,
    ) -> Result<()> {
        process_create_reply(ctx, random_seed, metadata_uri)
    }
    pub fn update_post(
        ctx: Context<UpdatePost>,
        random_seed: [u8; 32],
        metadata_uri: String,
    ) -> Result<()> {
        process_update_post(ctx, random_seed, metadata_uri)
    }
}
