use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{error::VaultError, UserVault, VaultConfig};

#[derive(Accounts)]
pub struct Withdraw<'info> {
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
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = config,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require_gte!(
            self.user_vault.balance,
            amount,
            VaultError::InsufficientBalance
        );

        // 1. Update user vault balance
        self.user_vault.balance = self
            .user_vault
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::ArithmeticError)?;

        Ok(())
    }
}
