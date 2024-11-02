#[cfg(test)]
mod tests;

mod state;
mod instructions;
use instructions::{initialize::initialize, contribute::contribute, checker::checker, refund::refund};

use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::ProgramResult;

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

pub const ID: [u8; 32] =
    five8_const::decode_32_const("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FunraiserInstruction::try_from(discriminator)? {
        FunraiserInstruction::Initialize => initialize(accounts, data),
        FunraiserInstruction::Contribute => contribute(accounts, data),
        FunraiserInstruction::Checker => checker(accounts, data),
        FunraiserInstruction::Refund => refund(accounts, data),
    }
}
