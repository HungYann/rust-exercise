use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::{entrypoint, msg};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::processor::Note;


entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_content: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let note_account = next_account_info(account_info_iter)?;

    if !note_account.is_writable {
        return Err(ProgramError::Custom(2));
    }

    let mut note_data = Note::new(note_account.data.to_string());

    note_data
        .update_content(new_content)
        .map_err(|e| ProgramError::from(e))?;

    msg!("Note content updated: {}", note_data.content);

    Ok(())
}