use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult};
use pinocchio_token::state::TokenAccount;
use chrono::Utc;

pub fn contribute(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    let [
        contributor, 
        mint_to_raise, 
        fundraiser, 
        contributor_account, 
        contributor_ta, 
        vault, 
        _token_program, 
        _system_program
    ] = accounts else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys)
    };

    assert!(contributor.is_signer());

    let mint: Pubkey;

    let current_time = Utc::now().timestamp();
    // ((fundraiser.durations <= current_time - fundraiser.timestarted) / 86400) as u8,
    unsafe {
        let amount = *(data.as_ptr() as *const u64);
        mint = Pubkey::default();// todo! Get getter from fundraiser
        assert_eq!(&mint, mint_to_raise.key());

        let contributor_ata = TokenAccount::from_account_info_unchecked_unsafe(contributor_ta);
        assert_eq!(&contributor_ata.mint(), mint_to_raise.key());
        assert_eq!(&contributor_ata.authority(), fundraiser.key());

        // contributor_account.amount <= (fundraiser.amount_to_raise * 10u64) / 100u64
        // contributor_account.amount + amount <= (fundraiser.amount_to_raise * 10u64) / 100u64)

        pinocchio_token::instructions::Transfer {
            from: contributor_ta,
            to: vault,
            authority: contributor,
            amount,
        }.invoke()?;
    }
    
    Ok(())
}