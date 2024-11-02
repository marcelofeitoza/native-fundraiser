use chrono::Utc;
use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, pubkey::{self, Pubkey}, ProgramResult};
use pinocchio_token::state::TokenAccount;

pub fn refund(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        contributor,
        maker,
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

    let current_time = Utc::now().timestamp();

    // fundraiser.duration >= ((current_time - fundraiser.timestarted) / 86400) as u8
    // vault.amount < fundraiser.amount_to_raise

    let binding = pubkey::find_program_address(&[b"fundraiser"], &fundraiser.key()).1;
    let bump_binding = binding.to_be_bytes();
    let seeds = [Seed::from(b"fundraiser"), Seed::from(maker.key().as_ref()), Seed::from(&bump_binding)];
    let signer = [Signer::from(&seeds)];

    let mint: Pubkey;
    
    unsafe {
        mint = Pubkey::default();// todo! Get getter from fundraiser
        assert_eq!(&mint, mint_to_raise.key());

        let contributor_ata = TokenAccount::from_account_info_unchecked_unsafe(contributor_ta);
        assert_eq!(&contributor_ata.mint(), mint_to_raise.key());
        assert_eq!(&contributor_ata.authority(), fundraiser.key());

        let vault_ta = TokenAccount::from_account_info_unchecked_unsafe(vault);

        pinocchio_token::instructions::Transfer {
            from: vault,
            to: contributor_ta,
            authority: fundraiser,
            amount: vault_ta.amount(),
        }.invoke_signed(&signer)?;

        let lamports = contributor_account.borrow_lamports_unchecked();
        *(contributor_account.borrow_mut_lamports_unchecked()) -= lamports;
        *(contributor.borrow_mut_lamports_unchecked()) += lamports;
    }


    Ok(())
}