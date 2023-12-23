use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Artwork,
};


/// Mint a new artwork as an NFT.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` new_artwork: [Artwork] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - new_artwork_seed_creator: [Pubkey] Auto-generated, from input new_artwork of type [Artwork] set the seed named creator, required by the type
pub fn mint_artwork(
	program_id: &Pubkey,
	new_artwork: &mut AccountPDA<Artwork>,
) -> ProgramResult {
    // Implement your business logic here...
    new_artwork.data.title = "New Artwork Title".to_string();
    new_artwork.data.description = "Description of the artwork".to_string();
    new_artwork.data.price = 100;
    
    Ok(())
}