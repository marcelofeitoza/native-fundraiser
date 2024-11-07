use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [fundraiser] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(fundraiser.is_signer());

    unsafe { *(fundraiser.borrow_mut_data_unchecked().as_ptr() as *mut &[u8]) = data };

    Ok(())
}
