#![allow(unexpected_cfgs)]
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use instructions::*;
use state::*;

declare_id!("8V7ou9PByAPVdbAetGAPbh3n4WqUVrPgDyzQgDjyTrNC");

#[program]
pub mod whitelist_vault {
    use super::*;

    pub fn init_config(ctx:Context<InitConfig>)->Result<()>{
        ctx.accounts.init(ctx.bumps)?;
        Ok(())
    }

    pub fn whitelist_user(ctx:Context<WhitelistOperations>,user:Pubkey) -> Result<()> {
        ctx.accounts.whitelist_user(user)?;
        Ok(())
    }

    pub fn blacklist_user(ctx:Context<WhitelistOperations>,user:Pubkey) -> Result<()> {
        ctx.accounts.blacklist_user(user)?;
        Ok(())
    }

    pub fn deposit(ctx:Context<VaultOps>,amount:u64) -> Result<()>{
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx:Context<VaultOps>) -> Result<()> {
        ctx.accounts.withdraw()?;
        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(amount)?;
        Ok(())
    }
    
}