use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::spl_token_2022::{
        self, extension::memo_transfer::instruction::enable_required_transfer_memos,
    }, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::VaultConfig;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + VaultConfig::INIT_SPACE,
        seeds = [b"vault-config"],
        bump
    )]
    pub vault_config: Account<'info, VaultConfig>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = vault_config,
        associated_token::token_program = token_program,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitConfig<'info> {
    pub fn init(&mut self, bumps: &InitConfigBumps) -> Result<()> {
        let vault_config = &mut self.vault_config;
        vault_config.admin = self.admin.key();
        vault_config.mint = self.mint.key();
        vault_config.vault_ata = self.vault_ata.key();
        vault_config.bump = bumps.vault_config;

        // Enable memo transfer requirement on the vault ATA
        self.enable_memo_transfers(bumps)?;

        Ok(())
    }

    fn enable_memo_transfers(&self, bumps: &InitConfigBumps) -> Result<()> {
        // Create the instruction to enable required transfer memos
        let enable_memo_ix = enable_required_transfer_memos(
            self.token_program.key,
            &self.vault_ata.key(),
            &self.vault_config.key(),
            &[],
        )?;

        // Create signer seeds for the vault_config PDA
        let seeds = &[
            b"vault-config".as_ref(),
            &[bumps.vault_config],
        ];
        let signer_seeds = &[&seeds[..]];

        // Invoke the instruction with PDA as signer
        anchor_lang::solana_program::program::invoke_signed(
            &enable_memo_ix,
            &[
                self.vault_ata.to_account_info(),
                self.vault_config.to_account_info(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }
}