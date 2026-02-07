use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::VaultConfig;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8+VaultConfig::INIT_SPACE,
        seeds=[b"vault-config".as_ref()],
        bump)]
    pub vault_config: Account<'info, VaultConfig>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = vault_config,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitConfig<'info> {
    pub fn handler(ctx: Context<Self>, bumps: InitConfigBumps) -> Result<()> {
        let vault_config = &mut ctx.accounts.vault_config;
        vault_config.admin = ctx.accounts.admin.key();
        vault_config.mint = ctx.accounts.mint.key();
        vault_config.vault_ata = ctx.accounts.vault_ata.key();
        vault_config.bump = bumps.vault_config;
        Ok(())
    }
}
