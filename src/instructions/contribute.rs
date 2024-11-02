use chrono::Utc;
use pinocchio::{account_info::AccountInfo, ProgramResult};
use pinocchio_token::state::TokenAccount;

use crate::state::{Contributor, Fundraiser};

pub fn contribute(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, fundraiser, contributor_account, contributor_ta, vault, _token_program] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);
    let vault_account = unsafe { TokenAccount::from_account_info_unsafe(vault) };

    assert_eq!(fundraiser_account.mint_to_raise(), vault_account.mint());
    assert_eq!(&vault_account.authority(), fundraiser.key());

    let amount = unsafe { *(data.as_ptr() as *const u64) };

    if contributor_account.data_len() != 0 {
        unsafe {
            *(contributor.borrow_mut_data_unchecked().as_mut_ptr()
                as *mut [u8; Contributor::LEN]) =
                (Contributor::from_account_info(contributor_account).amount() + amount)
                    .to_le_bytes();
        }
    } else {
        unsafe {
            *(contributor.borrow_mut_data_unchecked().as_mut_ptr()
                as *mut [u8; Contributor::LEN]) = amount.to_le_bytes();
        }
    }

    let current_time = Utc::now().timestamp();
    assert!(fundraiser_account.time_ending() > current_time);

    pinocchio_token::instructions::Transfer {
        from: contributor_ta,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    Ok(())
}
