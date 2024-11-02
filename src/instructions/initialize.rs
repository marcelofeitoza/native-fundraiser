use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    ProgramResult,
};
use pinocchio_token::instructions::InitilizeAccount3;

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [fundraiser, mint, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(fundraiser.is_signer());

    let (bump, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    unsafe {
        let data_ptr = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();

        // Copy `mint_to_raise` key to the first 32 bytes of `fundraiser`
        *(data_ptr as *mut [u8; 32]) = *mint.key();

        // Copy the remaining `data` bytes after the first 32 bytes in `fundraiser`
        *(data_ptr.add(32) as *mut [u8; 56]) = *(data.as_ptr() as *const [u8; 56]);
    }

    let binding = bump.to_le_bytes();
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&binding)];
    let signer = [Signer::from(&seeds)];

    // Create a Derived TA for the vault
    InitilizeAccount3 {
        token: vault,
        owner: fundraiser.key(),
        mint,
    }
    .invoke_signed(&signer)?;

    Ok(())
}
