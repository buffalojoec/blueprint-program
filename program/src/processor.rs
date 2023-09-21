//! Blueprint program state processor

use solana_program::{msg, entrypoint::ProgramResult, account_info::AccountInfo, pubkey::Pubkey};

/// Blueprint program state processor
pub fn process(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Blueprint program state processor");

    Ok(())
}