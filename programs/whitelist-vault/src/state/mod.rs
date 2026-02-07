use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct UserVault {
    pub offer: u64,
    pub whitelist_status: bool,
    pub bump: u8,
}

#[derive(InitSpace)]
#[account]
pub struct VaultConfig{
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub vault_ata: Pubkey,
    pub bump: u8,
}