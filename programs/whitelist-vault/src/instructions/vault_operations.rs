use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked,Mint, TokenAccount, TokenInterface,TransferChecked},
};

use crate::{UserVault, VaultConfig, error::VaultError};

#[derive(Accounts)]
pub struct VaultOps<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      mut,
      seeds=[b"user-vault",user.key().as_ref()],
      bump)]
    pub user_vault: Account<'info, UserVault>,
    #[account(
        seeds = [b"vault-config"],
        bump = config.bump
    )]
    pub config: Account<'info,VaultConfig>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user_vault,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> VaultOps<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.user_ata.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.user.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        let user_vault= &mut self.user_vault;
        let new_bal = user_vault.balance.checked_add(amount).ok_or(VaultError::ArithmaticError)?;
        user_vault.balance = new_bal;

        Ok(())
    }
    pub fn withdraw(&mut self) -> Result<()> {
        require_gt!(self.user_vault.balance, 0, VaultError::InsufficientBalance);
         let signer_seeds: [&[&[u8]]; 1] = [&[
            b"vault-config",
        ]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault_ata.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.config.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        transfer_checked(cpi_context, self.user_vault.balance, self.mint.decimals)?;
        self.user_vault.balance = 0;

        Ok(())
    }
}
