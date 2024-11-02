use solana_nostd_sha256::hashv;
use pinocchio::{account_info::AccountInfo, sysvars::{clock::Clock, Sysvar}, ProgramResult};
use pinocchio_token::{state::TokenAccount, instructions::Transfer};

use crate::{state::{Contributor, Fundraiser}, ID, PDA_MARKER};

pub fn contribute(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, fundraiser, contributor_account, contributor_ta, vault, _token_program] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    let (bump, data) = data
        .split_first()
        .ok_or(pinocchio::program_error::ProgramError::InvalidInstructionData)?;

    let pda = hashv(&[
        fundraiser.key().as_ref(),
        &[*bump],
        ID.as_ref(),
        PDA_MARKER,
    ]);

    assert_eq!(pda, vault.key().as_ref());

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);
    assert_eq!(fundraiser_account.mint_to_raise(), unsafe { TokenAccount::from_account_info_unsafe(vault).mint() });

    let amount = unsafe { *(data.as_ptr() as *const u64) };

    if contributor_account.data_len() != 0 {
        unsafe {
            *(contributor_account.borrow_mut_data_unchecked().as_mut_ptr()
                as *mut [u8; Contributor::LEN]) =
                (Contributor::from_account_info(contributor_account).amount() + amount)
                    .to_le_bytes();
        }
    } else {
        unsafe {
            *(contributor_account.borrow_mut_data_unchecked().as_mut_ptr()
                as *mut [u8; Contributor::LEN]) = amount.to_le_bytes();
        }
    }

    let current_time = Clock::get()?.unix_timestamp;
    assert!(fundraiser_account.time_ending() > current_time);

    Transfer {
        from: contributor_ta,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    Ok(())
}
