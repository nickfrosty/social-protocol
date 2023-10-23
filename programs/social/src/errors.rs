use anchor_lang::prelude::*;

#[error_code]
pub enum GenericError {
    #[msg("Invalid account")]
    InvalidAccount,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("The provided name string is too long")]
    NameTooLong,

    #[msg("The provided uri string is too long")]
    UriTooLong,

    #[msg("The provided uri is invalid")]
    InvalidUri,
}
