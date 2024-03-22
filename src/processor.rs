use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub enum NoteError {
    ContentTooLong,
    PermissionDenied,
}

pub struct Note {
    pub content: String,
}

impl Note {
    pub fn new(content: String) -> Note {
        Note { content }
    }

    pub fn update_content(&mut self, new_content: String) -> Result<(), NoteError> {
        if new_content.len() > 100 {
            return Err(NoteError::ContentTooLong);
        }
        self.content = new_content;
        Ok(())
    }
}





