use anchor_lang::prelude::*;

use crate::UserVault;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
      payer = admin,
      space = 8+UserVault::INIT_SPACE,
    seeds = [b"user-vault",user.as_ref()] ,
    bump,
  )]
    pub user_vault: Account<'info, UserVault>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn whitelist_user(&mut self, user: Pubkey,bump:WhitelistOperationsBumps) -> Result<()> {
        msg!("user ata {} whitelisted", user);
        let new_user_vault = &mut self.user_vault;
        if new_user_vault.bump == 0 {
            new_user_vault.balance = 0;
            new_user_vault.bump = bump.user_vault;
        }
        new_user_vault.whitelist_status = true;
        Ok(())
    }

    pub fn blacklist_user(&mut self, user: Pubkey) -> Result<()> {
        msg!("user ata {} balcklisted", user);
        let new_user_vault = &mut self.user_vault;
        new_user_vault.whitelist_status = false;
        Ok(())
    }
}
