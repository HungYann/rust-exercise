use solana_program::program_error::ProgramError;

pub enum NoteError {
    ContentTooLong,
    PermissionDenied,
}

impl From<NoteError> for ProgramError {
    fn from(e: NoteError) -> Self {
        match e {
            NoteError::ContentTooLong => ProgramError::Custom(1),
            NoteError::PermissionDenied => ProgramError::Custom(2),
        }
    }
}