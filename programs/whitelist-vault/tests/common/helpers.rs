use {
    litesvm::LiteSVM,
    litesvm_token::{get_spl_account, spl_token::state::Account},
    solana_sdk::{
        clock::Clock,
        message::{Instruction, Message},
        pubkey::Pubkey as Address,
        signature::Keypair,
        transaction::Transaction,
    },
};

use super::fixtures::*;
use anchor_lang::prelude::Pubkey;

use anchor_lang::prelude::AccountMeta as AnchorAccountMeta;
use solana_sdk::instruction::AccountMeta as SdkAccountMeta;

// Helper function to convert
pub fn convert_account_metas(anchor_metas: Vec<AnchorAccountMeta>) -> Vec<SdkAccountMeta> {
    anchor_metas
        .into_iter()
        .map(|meta| SdkAccountMeta {
            pubkey: meta.pubkey.to_address(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect()
}
/// Send a transaction to the LiteSVM
pub fn send_transaction(
    instruction: &[Instruction],
    svm: &mut LiteSVM,
    signers: &[Keypair],
    payer: Pubkey,
) {
    let message = Message::new(instruction, Some(&payer.to_address()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(signers, message, recent_blockhash);

    let result = svm
        .send_transaction(transaction)
        .expect("Transaction should succeed");

    println!("\n✅ Transaction successful");
    println!("   CUs consumed: {}", result.compute_units_consumed);
    println!("   Signature: {}", result.signature);
}

/// Assert vault account state
pub fn assert_vault_state(
    svm: &LiteSVM,
    vault: &Pubkey,
    expected_amount: u64,
    expected_owner: &Pubkey,
    expected_mint: &Pubkey,
) {
    let vault_data: Account = get_spl_account(svm, &Address::from(vault.to_bytes()))
        .expect("Should deserialize vault data");

    assert_eq!(
        vault_data.amount, expected_amount,
        "Vault amount mismatch: expected {}, got {}",
        expected_amount, vault_data.amount
    );
    assert_eq!(
        &vault_data.owner.to_pubkey(),
        expected_owner,
        "Vault owner mismatch"
    );
    assert_eq!(
        &vault_data.mint.to_pubkey(),
        expected_mint,
        "Vault mint mismatch"
    );
}

/// Assert vault account is closed (no data, no lamports)
pub fn assert_vault_closed(svm: &LiteSVM, vault: &Pubkey) {
    let vault_account = svm
        .get_account(&Address::from(vault.to_bytes()))
        .expect("Should be able to check vault account");

    assert_eq!(
        vault_account.data.len(),
        0,
        "Vault data should be empty after close"
    );
    assert_eq!(
        vault_account.lamports, 0,
        "Vault lamports should be zero after close"
    );
}

/// Assert escrow account state
// pub fn assert_escrow_state(
//     svm: &LiteSVM,
//     escrow: &Pubkey,
//     expected_seed: u64,
//     expected_maker: &Pubkey,
//     expected_mint_a: &Pubkey,
//     expected_mint_b: &Pubkey,
//     expected_receive: u64,
// ) {
//     let escrow_account = svm
//         .get_account(escrow)
//         .expect("Escrow account should exist");

//     let escrow_data = crate::state::VaultConfig::try_deserialize(&mut escrow_account.data.as_ref())
//         .expect("Should deserialize escrow data");

//     assert_eq!(escrow_data.seed, expected_seed, "Escrow seed mismatch");
//     assert_eq!(escrow_data.maker, *expected_maker, "Escrow maker mismatch");
//     assert_eq!(
//         escrow_data.mint_a, *expected_mint_a,
//         "Escrow mint_a mismatch"
//     );
//     assert_eq!(
//         escrow_data.mint_b, *expected_mint_b,
//         "Escrow mint_b mismatch"
//     );
//     assert_eq!(
//         escrow_data.receive, expected_receive,
//         "Escrow receive amount mismatch"
//     );
// }

/// Set the sysvar clock for time-based tests
pub fn set_clock(svm: &mut LiteSVM, slot: u64, epoch: u64, unix_timestamp: i64) {
    let mut clock: Clock = svm.get_sysvar();
    clock.slot = slot;
    clock.epoch = epoch;
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar(&clock);
}

/// Set clock to test values (uses constants from fixtures)
pub fn set_test_clock(svm: &mut LiteSVM) {
    set_clock(svm, TEST_SLOT, TEST_EPOCH, TEST_UNIX_TIMESTAMP);
}

pub fn get_pubkey_from_address(key: Pubkey) -> Address {
    Address::from(key.to_bytes())
}
