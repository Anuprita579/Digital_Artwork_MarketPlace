// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::generated::errors::DigitalArtMarketplaceError;

#[derive(BorshSerialize, Debug)]
pub enum DigitalArtMarketplaceInstruction {
/// Mint a new artwork as an NFT.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` new_artwork: [Artwork] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - new_artwork_seed_creator: [Pubkey] Auto-generated, from input new_artwork of type [Artwork] set the seed named creator, required by the type
	MintArtwork(MintArtworkArgs),

/// Purchase an artwork and transfer ownership.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` buyer_info: [CustomAccountInfo] 
/// 2. `[]` artwork_account_info: [CustomAccountInfo] 
/// 3. `[]` seller_info: [CustomAccountInfo] 
/// 4. `[]` fee_payer_info: [CustomAccountInfo] 
///
/// Data:
/// - artwork_id: [u64] The ID of the artwork to be purchased.
	PurchaseArtwork(PurchaseArtworkArgs),

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintArtworkArgs {
	pub new_artwork_seed_creator: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PurchaseArtworkArgs {
	pub artwork_id: u64,
}

impl DigitalArtMarketplaceInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(DigitalArtMarketplaceError::InvalidInstruction)?;

        Ok(match variant {
			0 => Self::MintArtwork(MintArtworkArgs::try_from_slice(rest).unwrap()),
			1 => Self::PurchaseArtwork(PurchaseArtworkArgs::try_from_slice(rest).unwrap()),
			_ => return Err(DigitalArtMarketplaceError::InvalidInstruction.into())
        })
    }
}