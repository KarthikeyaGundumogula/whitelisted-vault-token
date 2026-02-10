use anchor_lang::prelude::msg;
use anchor_spl::token_2022::spl_token_2022::{extension::StateWithExtensionsOwned, instruction::transfer_checked,state::Account};
use solana_sdk::{instruction::Instruction, message::AccountMeta, signer::Signer};

use crate::{
    builders::{
        create_mint_builder, init_config_builder, init_extra_acc_builder, mint_token_builder,
        whitelist_user_builder,
    },
    common::{builders::DepositInstructionBuilder, setup::Setup},
    helpers::send_transaction,
    ToAddress, ToPubkey, MEMO_PROGRAM_ID, MINT_AMOUNT, MINT_DECIMALS, PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID,
};

#[test]
fn test_deposit_success() {
    let mut setup = Setup::new();
    let admin = setup.admin.insecure_clone().pubkey().to_pubkey();

    // Create Mint FIRST (must exist before anything else)
    let create_mint_inx = create_mint_builder(&setup);
    send_transaction(
        &[create_mint_inx],
        &mut setup.svm,
        &[
            setup.admin.insecure_clone(),
            setup.mint_signer.insecure_clone(),
        ],
        admin,
    );
    msg!("mint created");

    // Init Vault Config (creates vault_ata for the mint)
    let init_inx = init_config_builder(&setup);
    send_transaction(
        &[init_inx],
        &mut setup.svm,
        &[setup.admin.insecure_clone()],
        admin,
    );
    msg!("vault initialized");

    // Add to whitelist
    let whitelist_inx = whitelist_user_builder(&setup);
    send_transaction(
        &[whitelist_inx],
        &mut setup.svm,
        &[setup.admin.insecure_clone()],
        admin,
    );
    msg!("user whitelisted");

    // Create user ATA after mint exists
    setup.create_user_ata();
    msg!("User ATA created");

    // Mint some tokens to the user
    let mint_inx = mint_token_builder(&setup);
    send_transaction(
        &[mint_inx],
        &mut setup.svm,
        &[setup.admin.insecure_clone()],
        admin,
    );
    msg!("minted some tokens to the user");

    // Init Extra Account metas
    let extra_accs = init_extra_acc_builder(&setup);
    send_transaction(
        &[extra_accs],
        &mut setup.svm,
        &[setup.admin.insecure_clone()],
        admin,
    );
    msg!("initialized extra account metas");

    // Create memo instruction
    let memo = format!(
        "deposit:{}:{}:{}",
        setup.user.pubkey(),
        MINT_AMOUNT,
        MINT_AMOUNT
    );

    let memo_ix = Instruction {
        program_id: MEMO_PROGRAM_ID.to_address(),
        accounts: vec![],
        data: memo.into_bytes(),
    };

    // Create transfer instruction
    let ix = transfer_checked(
        &TOKEN_2022_PROGRAM_ID,
        &setup.user_ata.to_pubkey(),
        &setup.mint.to_pubkey(),
        &setup.vault_ata.to_pubkey(),
        &setup.user.pubkey().to_pubkey(),
        &[],
        MINT_AMOUNT,
        MINT_DECIMALS,
    )
    .unwrap();

    let mut transfer_ix = Instruction {
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

    // Add Extra require accounts to the transaction
    transfer_ix.accounts.push(AccountMeta {
        pubkey: setup.vault_config,
        is_signer: false,
        is_writable: false,
    });

    transfer_ix.accounts.push(AccountMeta {
        pubkey: PROGRAM_ID.to_address(),
        is_signer: false,
        is_writable: false,
    });
    transfer_ix.accounts.push(AccountMeta {
        pubkey: setup.extra_acc_meta,
        is_signer: false,
        is_writable: false,
    });
    transfer_ix.accounts.push(AccountMeta {
        pubkey: setup.user_vault,
        is_signer: false,
        is_writable: false,
    });

    // Create deposit instruction
    let deposit_ix = DepositInstructionBuilder::new(&setup)
        .amount(MINT_AMOUNT)
        .nonce(MINT_AMOUNT)
        .build();

    send_transaction(
        &[memo_ix, transfer_ix, deposit_ix],
        &mut setup.svm,
        &[setup.user.insecure_clone()],
        setup.user.pubkey().to_pubkey(),
    );

    let vault_ata = setup.svm.get_account(&setup.vault_ata).unwrap();
    let vault_state = StateWithExtensionsOwned::<Account>::unpack(vault_ata.data).unwrap();
    assert_eq!(vault_state.base.amount,MINT_AMOUNT);
}
