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
    use spl_tlv_account_resolution::state::ExtraAccountMetaList;

    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }

    pub fn whitelist_user(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.whitelist_user(user,ctx.bumps)?;
        Ok(())
    }

    pub fn blacklist_user(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.blacklist_user(user)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, nonce: u64) -> Result<()> {
        ctx.accounts.deposit(amount, nonce)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
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
    pub fn init_transfer_hook(ctx: Context<InitExtraAccMeta>) -> Result<()> {
        msg!("Initializing Transfer Hook...");

        // Get the extra account metas for the transfer hook
        let extra_account_metas = InitExtraAccMeta::extra_account_metas()?;

        msg!("Extra Account Metas: {:?}", extra_account_metas);
        msg!("Extra Account Metas Length: {}", extra_account_metas.len());

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_acc_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )
        .unwrap();
        msg!("Transfer Hook Metadata Initialized!");
        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(amount)?;
        Ok(())
    }
}
