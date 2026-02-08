use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked,
};
use anchor_spl::token_interface::{
    Mint, TokenAccount, TokenInterface, 
};

use crate::{error::VaultError, UserVault, VaultConfig};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user-vault", user.key().as_ref()],
        bump = user_vault.bump
    )]
    pub user_vault: Account<'info, UserVault>,

    #[account(
        seeds = [b"vault-config"],
        bump = config.bump
    )]
    pub config: Account<'info, VaultConfig>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = config,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Instructions sysvar for memo introspection
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, nonce: u64) -> Result<()> {
        
        let expected_memo = format!("deposit:{}:{}:{}", self.user.key(), amount, nonce);
        
        
        Self::verify_memo_in_transaction(&self.instructions, &expected_memo)?;
        
        
        self.user_vault.balance = self
        .user_vault
        .balance
        .checked_add(amount)
        .ok_or(VaultError::ArithmeticError)?;
    
    Ok(())
}

fn verify_memo_in_transaction(
    instructions_sysvar: &AccountInfo,
    expected_memo: &str,
) -> Result<()> {
    // Get the current instruction index
    let current_index =
    load_current_index_checked(instructions_sysvar).map_err(|_| VaultError::InvalidMemo)?;
    
    // SPL Memo Program ID as a regular Pubkey
    let memo_program_id = Pubkey::new_from_array([
        5, 74, 83, 80, 248, 93, 200, 130, 214, 20, 165, 86, 114, 120, 138, 41, 109, 223, 30,
        171, 171, 208, 166, 6, 120, 136, 73, 50, 244, 238, 246, 160,
        ]);
        
        // Check all previous instructions for memo
        for i in 0..current_index {
            let ix = load_instruction_at_checked(i.into(), instructions_sysvar)
            .map_err(|_| VaultError::InvalidMemo)?;
        
        // Check if this instruction is from the memo program
        if ix.program_id == memo_program_id {
            // Parse memo data as UTF-8 string
            let memo_text =
            std::str::from_utf8(&ix.data).map_err(|_| VaultError::InvalidMemo)?;
            
            // Check if memo matches expected format
            if memo_text == expected_memo {
                return Ok(());
            }
        }
    }
    
    // No matching memo found
    Err(VaultError::MemoNotFound.into())
}
}
