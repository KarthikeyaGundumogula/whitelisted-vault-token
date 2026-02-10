use anchor_lang::{system_program::ID as SYSTEM_PROGRAM, InstructionData, ToAccountMetas};
use solana_sdk::{message::Instruction, signer::Signer};

use whitelist_vault;

use crate::helpers::convert_account_metas;

use super::{fixtures::*, setup::Setup};

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

    pub fn build(self) -> Instruction {
        let setup = self.setup;
        let anchor_accounts = whitelist_vault::accounts::Deposit {
            user: setup.user.pubkey().to_pubkey(),
            user_vault: setup.user_vault.to_pubkey(),
            config: setup.vault_config.to_pubkey(),
            mint: setup.mint.to_pubkey(),
            vault_ata: setup.vault_ata.to_pubkey(),
            instructions: SYSVAR_INSTRUCTION,
            token_program: TOKEN_2022_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM,
        }
        .to_account_metas(None);
        let sdk_accounts = convert_account_metas(anchor_accounts);
        let amount = self.amount;
        let nonce = self.nonce;

        let data = whitelist_vault::instruction::Deposit { amount, nonce }.data();

        Instruction {
            program_id: PROGRAM_ID.to_address(),
            accounts: sdk_accounts,
            data,
        }
    }
}

pub fn init_config_builder(setup: &Setup) -> Instruction {
    let anchor_accounts = whitelist_vault::accounts::InitConfig {
        admin: setup.admin.pubkey().to_pubkey(),
        mint: setup.mint.to_pubkey(),
        vault_ata: setup.vault_ata.to_pubkey(),
        vault_config: setup.vault_config.to_pubkey(),
        associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
        token_program: TOKEN_2022_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM,
    }
    .to_account_metas(None);
    let sdk_accounts = convert_account_metas(anchor_accounts);
    let data = whitelist_vault::instruction::InitConfig {}.data();
    Instruction {
        program_id: PROGRAM_ID.to_address(),
        accounts: sdk_accounts,
        data,
    }
}

pub fn init_extra_acc_builder(setup: &Setup) -> Instruction {
    let anchor_accounts = whitelist_vault::accounts::InitExtraAccMeta {
        signer: setup.admin.pubkey().to_pubkey(),
        extra_acc_meta_list: setup.extra_acc_meta.to_pubkey(),
        mint: setup.mint.to_pubkey(),
        system_program: SYSTEM_PROGRAM,
    }
    .to_account_metas(None);
    let sdk_accounts = convert_account_metas(anchor_accounts);
    let data = whitelist_vault::instruction::InitTransferHook {}.data();
    Instruction {
        program_id: PROGRAM_ID.to_address(),
        accounts: sdk_accounts,
        data,
    }
}

pub fn create_mint_builder(setup: &Setup) -> Instruction {
    let anchor_accounts = whitelist_vault::accounts::CreateMint {
        admin: setup.admin.pubkey().to_pubkey(),
        mint: setup.mint_signer.pubkey().to_pubkey(),
        token_program: TOKEN_2022_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM,
    }
    .to_account_metas(None);
    let sdk_accounts = convert_account_metas(anchor_accounts);
    let data = whitelist_vault::instruction::CreateMint {
        name: "kapten".to_string(),
        symbol: "JAL".to_string(),
        uri: "www.jal.sol".to_string(),
    }
    .data();

    Instruction {
        program_id: PROGRAM_ID.to_address(),
        accounts: sdk_accounts,
        data,
    }
}

pub fn mint_token_builder(setup: &Setup) -> Instruction {
    let anchor_accounts = whitelist_vault::accounts::MintToken {
        admin: setup.admin.pubkey().to_pubkey(),
        user: setup.user.pubkey().to_pubkey(),
        mint: setup.mint.to_pubkey(),
        user_ata: setup.user_ata.to_pubkey(),
        system_program: SYSTEM_PROGRAM,
        token_program: TOKEN_2022_PROGRAM_ID,
        associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
    }
    .to_account_metas(None);
    let sdk_accounts = convert_account_metas(anchor_accounts);
    let data = whitelist_vault::instruction::MintToken {
        amount: 2 * MINT_AMOUNT,
    }
    .data();

    Instruction {
        program_id: PROGRAM_ID.to_address(),
        accounts: sdk_accounts,
        data,
    }
}

pub fn whitelist_user_builder(setup: &Setup) -> Instruction {
    let anchor_accounts = whitelist_vault::accounts::WhitelistOperations {
        admin: setup.admin.pubkey().to_pubkey(),
        user_vault: setup.user_vault.to_pubkey(),
        system_program: SYSTEM_PROGRAM,
    }
    .to_account_metas(None);

    let sdk_accounts = convert_account_metas(anchor_accounts);
    let data = whitelist_vault::instruction::WhitelistUser {
        user: setup.user.pubkey().to_pubkey(),
    }
    .data();

    Instruction {
        program_id: PROGRAM_ID.to_address(),
        accounts: sdk_accounts,
        data,
    }
}