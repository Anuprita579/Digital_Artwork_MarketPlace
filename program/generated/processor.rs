// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::DigitalArtMarketplaceError;
use crate::generated::instructions::DigitalArtMarketplaceInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	Artwork,
	CustomAccountInfo,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = DigitalArtMarketplaceInstruction::unpack(data)?;

        match instruction {
			DigitalArtMarketplaceInstruction::MintArtwork(args) => {
				msg!("Instruction: MintArtwork");
				Self::process_mint_artwork(
					program_id,
					accounts, 
					args.new_artwork_seed_creator,
				)
			}
			DigitalArtMarketplaceInstruction::PurchaseArtwork(args) => {
				msg!("Instruction: PurchaseArtwork");
				Self::process_purchase_artwork(
					program_id,
					accounts, 
					args.artwork_id,
				)
			}
        }
    }

/// Mint a new artwork as an NFT.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` new_artwork: [Artwork] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - new_artwork_seed_creator: [Pubkey] Auto-generated, from input new_artwork of type [Artwork] set the seed named creator, required by the type
	pub fn process_mint_artwork(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		new_artwork_seed_creator: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let new_artwork_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (new_artwork_pubkey, new_artwork_bump) = Pubkey::find_program_address(
			&[b"artwork", new_artwork_seed_creator.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(DigitalArtMarketplaceError::InvalidSignerPermission.into());
		}

		if *new_artwork_info.key != new_artwork_pubkey {
			return Err(DigitalArtMarketplaceError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(DigitalArtMarketplaceError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		if new_artwork_info.lamports() == 0 && *new_artwork_info.owner == system_program::id() {
			let space = Artwork::LEN;
			let rent = Rent::get()?;
			let rent_minimum_balance = rent.minimum_balance(space);

			invoke_signed(
				&create_account(
					&fee_payer_info.key,
					&new_artwork_info.key,
					rent_minimum_balance,
					space as u64,
					program_id,
				),
				&[fee_payer_info.clone(), new_artwork_info.clone()],
				&[&[b"artwork", new_artwork_seed_creator.as_ref(), &[new_artwork_bump]]],
			)?;
		}


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(DigitalArtMarketplaceError::WrongAccountOwner.into());
		}

		if new_artwork_info.data_len() != Artwork::LEN {
			return Err(DigitalArtMarketplaceError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let new_artwork = &mut AccountPDA::new(
			&new_artwork_info,
			try_from_slice_unchecked::<Artwork>(&new_artwork_info.data.borrow()).unwrap(),
			new_artwork_bump,
		);

		// Calling STUB
		mint_artwork::mint_artwork(
			program_id,
			new_artwork,
		)?;

		// Accounts Serialization
		new_artwork.data.serialize(&mut &mut new_artwork_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_purchase_artwork(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		artwork_id: u64,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let buyer_info_info = next_account_info(account_info_iter)?;
		let artwork_account_info_info = next_account_info(account_info_iter)?;
		let seller_info_info = next_account_info(account_info_iter)?;
		let fee_payer_info_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (buyer_info_pubkey, buyer_info_bump) = Pubkey::find_program_address(
			&[b"account_info"],
			program_id,
		);
		let (artwork_account_info_pubkey, artwork_account_info_bump) = Pubkey::find_program_address(
			&[b"account_info"],
			program_id,
		);
		let (seller_info_pubkey, seller_info_bump) = Pubkey::find_program_address(
			&[b"account_info"],
			program_id,
		);
		let (fee_payer_info_pubkey, fee_payer_info_bump) = Pubkey::find_program_address(
			&[b"account_info"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(DigitalArtMarketplaceError::InvalidSignerPermission.into());
		}

		if *buyer_info_info.key != buyer_info_pubkey {
			return Err(DigitalArtMarketplaceError::NotExpectedAddress.into());
		}

		if *artwork_account_info_info.key != artwork_account_info_pubkey {
			return Err(DigitalArtMarketplaceError::NotExpectedAddress.into());
		}

		if *seller_info_info.key != seller_info_pubkey {
			return Err(DigitalArtMarketplaceError::NotExpectedAddress.into());
		}

		if *fee_payer_info_info.key != fee_payer_info_pubkey {
			return Err(DigitalArtMarketplaceError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(DigitalArtMarketplaceError::WrongAccountOwner.into());
		}

		if buyer_info_info.data_len() != CustomAccountInfo::LEN {
			return Err(DigitalArtMarketplaceError::InvalidAccountLen.into());
		}

		if artwork_account_info_info.data_len() != CustomAccountInfo::LEN {
			return Err(DigitalArtMarketplaceError::InvalidAccountLen.into());
		}

		if seller_info_info.data_len() != CustomAccountInfo::LEN {
			return Err(DigitalArtMarketplaceError::InvalidAccountLen.into());
		}

		if fee_payer_info_info.data_len() != CustomAccountInfo::LEN {
			return Err(DigitalArtMarketplaceError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let buyer_info = AccountPDA::new(
			&buyer_info_info,
			try_from_slice_unchecked::<CustomAccountInfo>(&buyer_info_info.data.borrow()).unwrap(),
			buyer_info_bump,
		);
		let artwork_account_info = AccountPDA::new(
			&artwork_account_info_info,
			try_from_slice_unchecked::<CustomAccountInfo>(&artwork_account_info_info.data.borrow()).unwrap(),
			artwork_account_info_bump,
		);
		let seller_info = AccountPDA::new(
			&seller_info_info,
			try_from_slice_unchecked::<CustomAccountInfo>(&seller_info_info.data.borrow()).unwrap(),
			seller_info_bump,
		);
		let fee_payer_info = AccountPDA::new(
			&fee_payer_info_info,
			try_from_slice_unchecked::<CustomAccountInfo>(&fee_payer_info_info.data.borrow()).unwrap(),
			fee_payer_info_bump,
		);

		// Calling STUB
		purchase_artwork::purchase_artwork(
			program_id,
			&buyer_info,
			&artwork_account_info,
			&seller_info,
			&fee_payer_info,
			artwork_id,
		)?;


		
		Ok(())
	}
}