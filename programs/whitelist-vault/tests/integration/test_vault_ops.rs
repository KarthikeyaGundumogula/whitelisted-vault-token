use anchor_lang::AccountDeserialize;
use anchor_spl::{token::spl_token::instruction::transfer_checked, };
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signer::Signer, transaction::Transaction
};

use crate::{TOKEN_PROGRAM_ID, ToAddress, ToPubkey, common::{
    builders::DepositInstructionBuilder,
    setup::Setup,
}};

fn setup_test() -> Setup{
    let setup = Setup::new();
     setup
}

#[test]
fn test_deposit_success() {
    let mut setup = setup_test();
    
    let amount = 1_000_000_u64; // 1 token (6 decimals)
    let nonce = 1_u64;
    
    // 1. Create memo instruction
    let memo = format!("deposit:{}:{}:{}", setup.user.pubkey(), amount, nonce);
    let memo_program_id = Pubkey::new_from_array([
        5, 74, 83, 80, 248, 93, 200, 130, 214, 20, 165, 86, 114, 120, 138, 41, 109, 223, 30,
        171, 171, 208, 166, 6, 120, 136, 73, 50, 244, 238, 246, 160,
        ]);
    let memo_ix = Instruction {
        program_id: memo_program_id,
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
        9,
    )
    .unwrap();

    let transfer_ix = Instruction {
        program_id: ix.program_id.to_address(),
        accounts: ix.accounts.into_iter().map(|meta| {
            solana_sdk::instruction::AccountMeta {
                pubkey: meta.pubkey.to_address(),
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            }
        }).collect(),
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
    assert!(result.is_ok(), "Deposit transaction failed: {:?}", result.err());
    
    // Verify user vault balance updated
    let user_vault_account = &setup.svm.get_account(&setup.user_vault).unwrap();
    let user_vault_data = whitelist_vault::state::UserVault::try_deserialize(
        &mut &user_vault_account.data[8..]
    ).unwrap();
    
    assert_eq!(user_vault_data.balance, amount, "User vault balance mismatch");
}

// #[test]
// fn test_deposit_without_memo_fails() {
//     let (mut context, setup) = setup_test();
    
//     let amount = 1_000_000_u64;
//     let nonce = 1_u64;
    
//     // Only transfer and deposit, NO memo
//     let transfer_ix = spl_token_2022::instruction::transfer_checked(
//         &TOKEN_2022_PROGRAM_ID,
//         &setup.user_token_account,
//         &setup.mint,
//         &setup.vault_ata,
//         &setup.user.pubkey(),
//         &[],
//         amount,
//         setup.decimals,
//     )
//     .unwrap();
    
//     let deposit_ix = DepositInstructionBuilder::new(&setup)
//         .amount(amount)
//         .nonce(nonce)
//         .build();
    
//     let tx = Transaction::new_signed_with_payer(
//         &[transfer_ix, deposit_ix],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     let result = context.send_transaction(tx);
//     assert!(result.is_err(), "Transaction should fail without memo");
// }

// #[test]
// fn test_deposit_with_wrong_memo_fails() {
//     let (mut context, setup) = setup_test();
    
//     let amount = 1_000_000_u64;
//     let nonce = 1_u64;
    
//     // Wrong memo format
//     let wrong_memo = format!("wrong:{}:{}:{}", setup.user.pubkey(), amount, nonce);
//     let memo_ix = Instruction {
//         program_id: spl_memo::ID,
//         accounts: vec![],
//         data: wrong_memo.into_bytes(),
//     };
    
//     let transfer_ix = spl_token_2022::instruction::transfer_checked(
//         &TOKEN_2022_PROGRAM_ID,
//         &setup.user_token_account,
//         &setup.mint,
//         &setup.vault_ata,
//         &setup.user.pubkey(),
//         &[],
//         amount,
//         setup.decimals,
//     )
//     .unwrap();
    
//     let deposit_ix = DepositInstructionBuilder::new(&setup)
//         .amount(amount)
//         .nonce(nonce)
//         .build();
    
//     let tx = Transaction::new_signed_with_payer(
//         &[memo_ix, transfer_ix, deposit_ix],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     let result = context.send_transaction(tx);
//     assert!(result.is_err(), "Transaction should fail with wrong memo");
// }

// #[test]
// fn test_deposit_amount_mismatch_fails() {
//     let (mut context, setup) = setup_test();
    
//     let transfer_amount = 1_000_000_u64;
//     let claimed_amount = 2_000_000_u64; // Different amount
//     let nonce = 1_u64;
    
//     // Memo says 2M but transfer is 1M
//     let memo = format!("deposit:{}:{}:{}", setup.user.pubkey(), claimed_amount, nonce);
//     let memo_ix = Instruction {
//         program_id: spl_memo::ID,
//         accounts: vec![],
//         data: memo.into_bytes(),
//     };
    
//     let transfer_ix = spl_token_2022::instruction::transfer_checked(
//         &TOKEN_2022_PROGRAM_ID,
//         &setup.user_token_account,
//         &setup.mint,
//         &setup.vault_ata,
//         &setup.user.pubkey(),
//         &[],
//         transfer_amount, // Different from memo
//         setup.decimals,
//     )
//     .unwrap();
    
//     let deposit_ix = DepositInstructionBuilder::new(&setup)
//         .amount(claimed_amount)
//         .nonce(nonce)
//         .build();
    
//     let tx = Transaction::new_signed_with_payer(
//         &[memo_ix, transfer_ix, deposit_ix],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     let result = context.send_transaction(tx);
//     assert!(result.is_err(), "Transaction should fail with amount mismatch");
// }

// #[test]
// fn test_deposit_multiple_times_different_nonces() {
//     let (mut context, setup) = setup_test();
    
//     // First deposit
//     let amount1 = 1_000_000_u64;
//     let nonce1 = 1_u64;
    
//     let memo1 = format!("deposit:{}:{}:{}", setup.user.pubkey(), amount1, nonce1);
//     let memo_ix1 = Instruction {
//         program_id: spl_memo::ID,
//         accounts: vec![],
//         data: memo1.into_bytes(),
//     };
    
//     let transfer_ix1 = spl_token_2022::instruction::transfer_checked(
//         &TOKEN_2022_PROGRAM_ID,
//         &setup.user_token_account,
//         &setup.mint,
//         &setup.vault_ata,
//         &setup.user.pubkey(),
//         &[],
//         amount1,
//         setup.decimals,
//     )
//     .unwrap();
    
//     let deposit_ix1 = DepositInstructionBuilder::new(&setup)
//         .amount(amount1)
//         .nonce(nonce1)
//         .build();
    
//     let tx1 = Transaction::new_signed_with_payer(
//         &[memo_ix1, transfer_ix1, deposit_ix1],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     context.send_transaction(tx1).unwrap();
    
//     // Second deposit with different nonce
//     let amount2 = 500_000_u64;
//     let nonce2 = 2_u64;
    
//     let memo2 = format!("deposit:{}:{}:{}", setup.user.pubkey(), amount2, nonce2);
//     let memo_ix2 = Instruction {
//         program_id: spl_memo::ID,
//         accounts: vec![],
//         data: memo2.into_bytes(),
//     };
    
//     let transfer_ix2 = spl_token_2022::instruction::transfer_checked(
//         &TOKEN_2022_PROGRAM_ID,
//         &setup.user_token_account,
//         &setup.mint,
//         &setup.vault_ata,
//         &setup.user.pubkey(),
//         &[],
//         amount2,
//         setup.decimals,
//     )
//     .unwrap();
    
//     let deposit_ix2 = DepositInstructionBuilder::new(&setup)
//         .amount(amount2)
//         .nonce(nonce2)
//         .build();
    
//     let tx2 = Transaction::new_signed_with_payer(
//         &[memo_ix2, transfer_ix2, deposit_ix2],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     context.send_transaction(tx2).unwrap();
    
//     // Verify total balance
//     let user_vault_account = context.get_account(&setup.user_vault).unwrap();
//     let user_vault_data = whitelist_vault::state::UserVault::try_from_slice(
//         &user_vault_account.data[8..]
//     ).unwrap();
    
//     assert_eq!(
//         user_vault_data.balance,
//         amount1 + amount2,
//         "User vault balance should be sum of both deposits"
//     );
// }

// #[test]
// fn test_withdraw_success() {
//     let (mut context, setup) = setup_test();
    
//     // First deposit some tokens
//     let deposit_amount = 2_000_000_u64;
//     let nonce = 1_u64;
    
//     let memo = format!("deposit:{}:{}:{}", setup.user.pubkey(), deposit_amount, nonce);
//     let memo_ix = Instruction {
//         program_id: spl_memo::ID,
//         accounts: vec![],
//         data: memo.into_bytes(),
//     };
    
//     let transfer_ix = spl_token_2022::instruction::transfer_checked(
//         &TOKEN_2022_PROGRAM_ID,
//         &setup.user_token_account,
//         &setup.mint,
//         &setup.vault_ata,
//         &setup.user.pubkey(),
//         &[],
//         deposit_amount,
//         setup.decimals,
//     )
//     .unwrap();
    
//     let deposit_ix = DepositInstructionBuilder::new(&setup)
//         .amount(deposit_amount)
//         .nonce(nonce)
//         .build();
    
//     let deposit_tx = Transaction::new_signed_with_payer(
//         &[memo_ix, transfer_ix, deposit_ix],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     context.send_transaction(deposit_tx).unwrap();
    
//     // Now withdraw
//     let withdraw_amount = 1_000_000_u64;
    
//     let withdraw_ix = WithdrawInstructionBuilder::new(&setup)
//         .amount(withdraw_amount)
//         .build();
    
//     let withdraw_tx = Transaction::new_signed_with_payer(
//         &[withdraw_ix],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     let result = context.send_transaction(withdraw_tx);
//     assert!(result.is_ok(), "Withdraw transaction failed: {:?}", result.err());
    
//     // Verify balance
//     let user_vault_account = context.get_account(&setup.user_vault).unwrap();
//     let user_vault_data = whitelist_vault::state::UserVault::try_from_slice(
//         &user_vault_account.data[8..]
//     ).unwrap();
    
//     assert_eq!(
//         user_vault_data.balance,
//         deposit_amount - withdraw_amount,
//         "Balance after withdrawal incorrect"
//     );
// }

// #[test]
// fn test_withdraw_insufficient_balance_fails() {
//     let (mut context, setup) = setup_test();
    
//     // Try to withdraw without any deposit
//     let withdraw_amount = 1_000_000_u64;
    
//     let withdraw_ix = WithdrawInstructionBuilder::new(&setup)
//         .amount(withdraw_amount)
//         .build();
    
//     let withdraw_tx = Transaction::new_signed_with_payer(
//         &[withdraw_ix],
//         Some(&setup.user.pubkey()),
//         &[&setup.user],
//         context.latest_blockhash(),
//     );
    
//     let result = context.send_transaction(withdraw_tx);
//     assert!(result.is_err(), "Withdraw should fail with insufficient balance");
// }