use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::UserVault;

#[derive(Accounts)]
pub struct VaultOps<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      init_if_needed,
      payer = user,
      space = 8+UserVault::INIT_SPACE,
      seeds=[b"user-vault",user_ata.key().as_ref()],
      bump)]
    pub vault: Account<'info, UserVault>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
        token::authority = user,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = vault,
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
