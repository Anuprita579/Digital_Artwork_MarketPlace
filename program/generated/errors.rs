// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum DigitalArtMarketplaceError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<DigitalArtMarketplaceError> for ProgramError {
    fn from(e: DigitalArtMarketplaceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for DigitalArtMarketplaceError {
    fn type_of() -> &'static str {
        "DigitalArtMarketplaceError"
    }
}

impl PrintProgramError for DigitalArtMarketplaceError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            DigitalArtMarketplaceError::InvalidInstruction => msg!("Error: Invalid instruction"),
            DigitalArtMarketplaceError::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            DigitalArtMarketplaceError::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            DigitalArtMarketplaceError::WrongAccountOwner => msg!("Error: Wrong account owner"),
            DigitalArtMarketplaceError::InvalidAccountLen => msg!("Error: Invalid account length"),
            DigitalArtMarketplaceError::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}