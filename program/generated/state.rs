// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug)]
pub struct Account<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
}

#[derive(Clone, Debug)]
pub struct AccountPDA<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
    pub bump: u8,
}

impl<'a, 'b, T> Account<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T) -> Self {
        Self { data, info }
    }
}

impl<'a, 'b, T> AccountPDA<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T, bump: u8) -> Self {
        Self { data, info, bump }
    }
}

/// Represents a digital artwork as an NFT.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct Artwork {
	pub title: String,
	pub description: String,
	pub price: u64,
	pub creator: Pubkey,
}

impl Artwork {
	pub const LEN: usize = 298; 
	}

/// Represents Solana account information.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct CustomAccountInfo {
	pub lamports: u64,
	pub owner: Pubkey,
}

impl CustomAccountInfo {
	pub const LEN: usize = 40; 
	}

