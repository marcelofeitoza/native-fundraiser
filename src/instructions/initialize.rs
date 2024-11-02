use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, ProgramResult
};

use crate::state::Fundraiser;

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [fundraiser, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(fundraiser.is_signer());

    // Fill the fundraiser account with the data
    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut [u8; Fundraiser::LEN]) =
            *(data.as_ptr() as *const [u8; Fundraiser::LEN]);
    }

    Ok(())
}