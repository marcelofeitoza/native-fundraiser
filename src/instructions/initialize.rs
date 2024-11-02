use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, mint_to_raise, fundraiser, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(fundraiser.is_signer());

    // Fill the fundraiser account with the data
    unsafe { 
        let data_ptr = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();
    
        // Copy maker key (first 32 bytes)
        *(data_ptr as *mut Pubkey) = *maker.key();
        // Copy mint_to_raise key (next 32 bytes)
        *(data_ptr.add(32) as *mut Pubkey) = *mint_to_raise.key();
        // Copy everything after mint_to_raise
        *(data_ptr.add(64) as *mut [u8; 25]) = *(data.as_ptr() as *const [u8; 25]);
    };

    Ok(())
}