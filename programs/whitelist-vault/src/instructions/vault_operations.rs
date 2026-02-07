use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{UserVault, VaultConfig};

#[derive(Accounts)]
pub struct VaultOps<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      init_if_needed,
      payer = user,
      space = 8+UserVault::INIT_SPACE,
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
        token::authority = user,
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
    pub fn deposit(self, _amount: u64) -> Result<()> {
        Ok(())
    }
    pub fn withdraw(self, _amount: u64) -> Result<()> {
        Ok(())
    }
}
