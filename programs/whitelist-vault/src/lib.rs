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
        ctx.accounts.init(&ctx.bumps)?;
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

    pub fn deposit(ctx:Context<Deposit>,amount:u64,nonce:u64) -> Result<()>{
        ctx.accounts.deposit(amount,nonce)?;
        Ok(())
    }

    pub fn withdraw(ctx:Context<Withdraw>,amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
    
    pub fn create_mint(
        ctx: Context<CreateMint>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.create_mint(name, symbol, uri)?;
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        ctx.accounts.mint(amount)?;
        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(amount)?;
        Ok(())
    }
    
}