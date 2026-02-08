use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Arithmetic overflow occurred")]
    ArithmeticError,
    #[msg("Insufficient balance for withdrawal")]
    InsufficientBalance,
    #[msg("Required memo not found in transaction")]
    MemoNotFound,
    #[msg("Invalid memo format")]
    InvalidMemo
}
