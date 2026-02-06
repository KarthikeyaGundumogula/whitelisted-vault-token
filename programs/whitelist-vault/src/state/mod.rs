use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct UserVault {
    pub offer: u64,
    pub offer_mint: Pubkey,
    pub whitelist_status: bool,
    pub bump: u8,
}
