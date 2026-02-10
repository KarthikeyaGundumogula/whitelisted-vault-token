use anchor_lang::AccountDeserialize;
use anchor_spl::{
    associated_token::spl_associated_token_account,
    token_2022::spl_token_2022::{
        self,
        extension::{
            metadata_pointer::MetadataPointer, transfer_hook::TransferHook,
            BaseStateWithExtensions, StateWithExtensionsOwned,
        },
        instruction::transfer_checked,
        state::{Account, Mint},
    },
    token_interface::{spl_token_metadata_interface::state::TokenMetadata, TokenAccount},
};
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signer::Signer, transaction::Transaction,
};

use crate::{
    common::{builders::DepositInstructionBuilder, setup::Setup},
    ToAddress, ToPubkey, MEMO_PROGRAM_ID, MINT_DECIMALS, TOKEN_PROGRAM_ID,
};

#[test]
fn test_deposit_success() {
    let mut setup = Setup::new();

    let amount = 1_000_000_u64; // 1 token (6 decimals)
    let nonce = 1_u64;

    // 1. Create memo instruction
    let memo = format!("deposit:{}:{}:{}", setup.user.pubkey(), amount, nonce);

    let memo_ix = Instruction {
        program_id: MEMO_PROGRAM_ID.to_address(),
        accounts: vec![],
        data: memo.into_bytes(),
    };

    // 2. Create transfer instruction
    let ix = transfer_checked(
        &TOKEN_PROGRAM_ID,
        &setup.user_ata.to_pubkey(),
        &setup.mint.to_pubkey(),
        &setup.vault_ata.to_pubkey(),
        &setup.user.pubkey().to_pubkey(),
        &[],
        amount,
        MINT_DECIMALS,
    )
    .unwrap();

    let transfer_ix = Instruction {
        program_id: ix.program_id.to_address(),
        accounts: ix
            .accounts
            .into_iter()
            .map(|meta| solana_sdk::instruction::AccountMeta {
                pubkey: meta.pubkey.to_address(),
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            })
            .collect(),
        data: ix.data,
    };

    // 3. Create deposit instruction
    let deposit_ix = DepositInstructionBuilder::new(&setup)
        .amount(amount)
        .nonce(nonce)
        .build();

    // Build and send transaction
    let tx = Transaction::new_signed_with_payer(
        &[memo_ix, transfer_ix, deposit_ix],
        Some(&setup.user.pubkey()),
        &[&setup.user],
        setup.svm.latest_blockhash(),
    );

    let result = setup.svm.send_transaction(tx);
    assert!(
        result.is_ok(),
        "Deposit transaction failed: {:?}",
        result.err()
    );

    // Verify user vault balance updated
    let user_vault_account = &setup.svm.get_account(&setup.user_vault).unwrap();
    let user_vault_data =
        whitelist_vault::state::UserVault::try_deserialize(&mut &user_vault_account.data[8..])
            .unwrap();

    assert_eq!(
        user_vault_data.balance, amount,
        "User vault balance mismatch"
    );
}
