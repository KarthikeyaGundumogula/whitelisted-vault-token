use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Arithmatic operation failed")]
    ArithmaticError,
    #[msg("Insufficient balance in vault")]
    InsufficientBalance,
}
