use anchor_spl::associated_token::get_associated_token_address_with_program_id;
use litesvm::LiteSVM;
use litesvm_token::{CreateAssociatedTokenAccount};
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

        // Create mint keypair but don't create ATAs yet
        let mint_signer = Keypair::new();
        let mint = mint_signer.pubkey();

        // Derive PDAs
        let vault_config = Pubkey::find_program_address(&[b"vault-config"], &program_id).0;

        let user_vault =
            Pubkey::find_program_address(&[b"user-vault", user.pubkey().as_ref()], &program_id).0;

        let extra_acc_meta =
            Pubkey::find_program_address(&[b"extra-account-metas", mint.as_ref()], &program_id).0;

        // Derive vault_ata address (will be created by init_config)
        let vault_ata = get_associated_token_address_with_program_id(
            &vault_config.to_pubkey(),
            &mint.to_pubkey(),
            &TOKEN_2022_PROGRAM_ID
        ).to_address();

        Setup {
            svm,
            admin,
            user,
            mint_signer,
            mint,
            user_ata: Pubkey::default(), // Will be set after mint creation
            vault_ata,
            vault_config,
            extra_acc_meta,
            user_vault,
        }
    }

    /// Create user ATA after mint has been created
    /// Note: vault_ata is created by init_config instruction
    pub fn create_user_ata(&mut self) {
        self.user_ata = Self::create_ata(&mut self.svm, &self.admin, &self.mint, &self.user.pubkey());
    }

    fn load_program(svm: &mut LiteSVM, program_id: Pubkey) {
        let so_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/deploy/whitelist_vault.so");
        let program_data = std::fs::read(so_path).expect("Failed to read program SO file");
        let _ = svm.add_program(program_id, &program_data);
    }

    fn create_ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
        CreateAssociatedTokenAccount::new(svm, payer, mint)
            .owner(owner)
            .token_program_id(&TOKEN_2022_PROGRAM_ID.to_address())
            .send()
            .expect("Failed to create associated token account")
    }
}