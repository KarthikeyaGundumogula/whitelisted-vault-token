use anchor_lang::{InstructionData, ToAccountMetas, system_program::ID as SYSTEM_PROGRAM};
use solana_sdk::{message::Instruction, signer::Signer};

use whitelist_vault;

use crate::helpers::convert_account_metas;

use super::{fixtures::*, setup::Setup,};

/// Builder for Make instruction
pub struct DepositInstructionBuilder<'a> {
    setup: &'a Setup,
    amount: u64,
    nonce: u64,
}

impl<'a> DepositInstructionBuilder<'a> {
    pub fn new(setup: &'a Setup) -> Self {
        Self {
            setup,
            amount: 0,
            nonce: 0,
        }
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = amount;
        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = nonce;
        self
    }

    pub fn build(self) -> Instruction{
        let setup = self.setup;
        let anchor_accounts = whitelist_vault::accounts::Deposit{
            user:setup.user.pubkey().to_pubkey(),
            user_vault: setup.user_vault.to_pubkey(),
            config:setup.vault_config.to_pubkey(),
            mint:setup.mint.to_pubkey(),
            vault_ata:setup.vault_ata.to_pubkey(),
            instructions:SYSVAR_INSTRUCTION,
            token_program:TOKEN_PROGRAM_ID,
            system_program:SYSTEM_PROGRAM

        }.to_account_metas(None);
        let sdk_accounts = convert_account_metas(anchor_accounts);
    let amount = self.amount;
    let nonce = self.nonce;
    
    let data = whitelist_vault::instruction::Deposit {
        amount,
        nonce,
    }.data();
    
    Instruction {
        program_id: PROGRAM_ID.to_address(),
        accounts: sdk_accounts,
        data,
    }
    }
}

