use solana_program::account_info:: AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;

use crate::generated::state::{
	AccountPDA,
	CustomAccountInfo,
};



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
pub fn purchase_artwork(
	program_id: &Pubkey,
	buyer_info: &AccountPDA<CustomAccountInfo>,
	artwork_account_info: &AccountPDA<CustomAccountInfo>,
	seller_info: &AccountPDA<CustomAccountInfo>,
	fee_payer_info: &AccountPDA<CustomAccountInfo>,
	artwork_id: u64,
) -> ProgramResult {
    // Implement your business logic here...

    // Check if the buyer is the fee payer (prevent others from buying on behalf of someone else)
    if buyer_info.info.key != fee_payer_info.info.key {
        return Err(ProgramError::InvalidArgument);
    }

    // Load the current owner (seller) from the artwork account data
    let mut artwork_data = artwork_account_info.info.try_borrow_mut_data()?;

    let current_owner: Pubkey = Pubkey::new(&artwork_data[0..32]);


    // Check if the current owner (seller) matches the expected seller account
    if current_owner != *seller_info.info.key {
        return Err(ProgramError::InvalidArgument);
    }

    // For simplicity, let's assume the artwork has a price field.
    let price: u64 = 100; // Replace with the actual logic to get the artwork price.

      // Deduct funds from the buyer
      **buyer_info.info.lamports.borrow_mut() -= price;


      // Add funds to the seller
      **seller_info.info.lamports.borrow_mut() += price;

    // Transfer ownership by updating the owner field in the artwork account
    artwork_data[0..32].copy_from_slice(buyer_info.info.key.as_ref());


    Ok(())
}