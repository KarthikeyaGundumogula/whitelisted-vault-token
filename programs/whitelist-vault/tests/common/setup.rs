use litesvm::LiteSVM;
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::path::PathBuf;

use whitelist_vault::ID;

use super::fixtures::*;

pub struct Setup {
    pub svm: LiteSVM,
    pub admin: Keypair,
    pub user: Keypair,
    pub mint_signer: Keypair,
    pub mint: Pubkey,
    pub user_ata: Pubkey,
    pub vault_ata: Pubkey,
    pub vault_config: Pubkey,
    pub extra_acc_meta: Pubkey,
    pub user_vault: Pubkey,
}

impl Setup {
    pub fn new() -> Self {
        let mut svm = LiteSVM::new();
        let admin = Keypair::new();
        let user = Keypair::new();

        // Airdrop SOL to payer
        svm.airdrop(&admin.pubkey(), AIRDROP_AMOUNT)
            .expect("Failed to airdrop SOL to payer");

        let program_id = Pubkey::from_str_const(&ID.to_string());

        // Load program
        Self::load_program(&mut svm, program_id);

        // Create mints
        let mint_signer = Keypair::new();
        let mint = mint_signer.pubkey();

        // Create associated token accounts
        let user_ata = Self::create_ata(&mut svm, &user, &mint, &user.pubkey());

        // Derive PDAs
        let vault_config = Pubkey::find_program_address(&[b"vault-config"], &program_id).0;

        let user_vault =
            Pubkey::find_program_address(&[b"vault-config", user.pubkey().as_ref()], &program_id).0;

        let extra_acc_meta =
            Pubkey::find_program_address(&[b"extra-account-metas", mint.as_ref()], &program_id).0;

        let vault_ata = Self::create_ata(&mut svm, &admin, &mint, &vault_config);
        // Mint initial tokens
        Self::mint_initial_tokens(&mut svm, &user, &mint, &user_ata);

        Setup {
            svm,
            admin,
            user,
            mint_signer,
            mint,
            user_ata,
            vault_ata,
            vault_config,
            extra_acc_meta,
            user_vault,
        }
    }

    fn load_program(svm: &mut LiteSVM, program_id: Pubkey) {
        let so_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/deploy/whitelist_vault.so");
        let program_data = std::fs::read(so_path).expect("Failed to read program SO file");
        let _ = svm.add_program(program_id, &program_data);
    }

    fn create_ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
        CreateAssociatedTokenAccount::new(svm, payer, mint)
            .owner(owner)
            .send()
            .expect("Failed to create associated token account")
    }

    fn mint_initial_tokens(svm: &mut LiteSVM, user: &Keypair, mint: &Pubkey, user_ata: &Pubkey) {
        MintTo::new(svm, user, mint, user_ata, MINT_AMOUNT)
            .send()
            .expect("Failed to mint tokens to maker ATA A");
    }
}
