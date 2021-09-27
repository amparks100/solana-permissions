use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum PermissionError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Permission Invalid")]
    InvalidPermission,
}

impl From<PermissionError> for ProgramError {
    fn from(e: PermissionError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
