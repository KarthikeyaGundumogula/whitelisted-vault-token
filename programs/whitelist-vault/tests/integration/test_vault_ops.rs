use anchor_lang::{AccountDeserialize, prelude::msg};
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
    builders::{create_mint_builder, init_config_builder, whitelist_user_builder},
    common::{builders::DepositInstructionBuilder, setup::Setup},
    helpers::send_transaction,
    ToAddress, ToPubkey, MEMO_PROGRAM_ID, MINT_AMOUNT, MINT_DECIMALS, TOKEN_PROGRAM_ID,
};

#[test]
fn test_deposit_success() {
    let mut setup = Setup::new();
    let admin = setup.admin.insecure_clone().pubkey().to_pubkey();

    // 1. Create Mint FIRST (must exist before anything else)
    let create_mint_inx = create_mint_builder(&setup);
    send_transaction(
        create_mint_inx,
        &mut setup.svm,
        &[setup.admin.insecure_clone(), setup.mint_signer.insecure_clone()],
        admin,
    );
    msg!("mint created");

    // 2. Init Vault Config (creates vault_ata for the mint)
    let init_inx = init_config_builder(&setup);
    send_transaction(
        init_inx,
        &mut setup.svm,
        &[setup.admin.insecure_clone()],
        admin,
    );
    msg!("vault initialized");
    
    // 3. Add to whitelist
    let whitelist_inx = whitelist_user_builder(&setup);
    send_transaction(
        whitelist_inx,
        &mut setup.svm,
        &[setup.admin.insecure_clone()],
        admin,
    );
    msg!("user whitelisted");

    // 4. Create user ATA after mint exists
    setup.create_user_ata();
    msg!("User ATA created");

    // // Create memo instruction
    // let memo = format!(
    //     "deposit:{}:{}:{}",
    //     setup.user.pubkey(),
    //     MINT_AMOUNT,
    //     MINT_AMOUNT
    // );

    // let memo_ix = Instruction {
    //     program_id: MEMO_PROGRAM_ID.to_address(),
    //     accounts: vec![],
    //     data: memo.into_bytes(),
    // };

    // // 2. Create transfer instruction
    // let ix = transfer_checked(
    //     &TOKEN_PROGRAM_ID,
    //     &setup.user_ata.to_pubkey(),
    //     &setup.mint.to_pubkey(),
    //     &setup.vault_ata.to_pubkey(),
    //     &setup.user.pubkey().to_pubkey(),
    //     &[],
    //     MINT_AMOUNT,
    //     MINT_DECIMALS,
    // )
    // .unwrap();

    // let transfer_ix = Instruction {
    //     program_id: ix.program_id.to_address(),
    //     accounts: ix
    //         .accounts
    //         .into_iter()
    //         .map(|meta| solana_sdk::instruction::AccountMeta {
    //             pubkey: meta.pubkey.to_address(),
    //             is_signer: meta.is_signer,
    //             is_writable: meta.is_writable,
    //         })
    //         .collect(),
    //     data: ix.data,
    // };

    // // 3. Create deposit instruction
    // let deposit_ix = DepositInstructionBuilder::new(&setup)
    //     .amount(MINT_AMOUNT)
    //     .nonce(MINT_AMOUNT)
    //     .build();

    // // Build and send transaction
    // let tx = Transaction::new_signed_with_payer(
    //     &[memo_ix, transfer_ix, deposit_ix],
    //     Some(&setup.user.pubkey()),
    //     &[&setup.user],
    //     setup.svm.latest_blockhash(),
    // );

    // let result = setup.svm.send_transaction(tx);
    // assert!(
    //     result.is_ok(),
    //     "Deposit transaction failed: {:?}",
    //     result.err()
    // );

    // // Verify user vault balance updated
    // let user_vault_account = &setup.svm.get_account(&setup.user_vault).unwrap();
    // let user_vault_data =
    //     whitelist_vault::state::UserVault::try_deserialize(&mut &user_vault_account.data[8..])
    //         .unwrap();

    // assert_eq!(
    //     user_vault_data.balance, MINT_AMOUNT,
    //     "User vault balance mismatch"
    // );
}