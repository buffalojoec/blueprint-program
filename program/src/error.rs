//! Blueprint program error types

use spl_program_error::*;

/// Errors that may be returned by the program.
#[spl_program_error]
pub enum BlueprintProgramError {
    /// Invalid schema format
    #[error("Invalid schema format")]
    InvalidFormat,
}
