use anchor_lang::prelude::Pubkey;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey as Address};

pub const SEED: u64 = 123;
pub const DEPOSIT_AMOUNT: u64 = 10;
pub const RECEIVE_AMOUNT: u64 = 10;
pub const MINT_DECIMALS: u8 = 6;
pub const MINT_AMOUNT: u64 = 1_000_000_000; // 1000 tokens with 6 decimals
pub const INITIAL_SOL: u64 = 100 * LAMPORTS_PER_SOL;
pub const AIRDROP_AMOUNT: u64 = 10 * LAMPORTS_PER_SOL;

// Test clock values for time-sensitive tests
pub const TEST_SLOT: u64 = 10;
pub const TEST_EPOCH: u64 = 1000;
pub const TEST_UNIX_TIMESTAMP: i64 = 6 * 86_400; // 6 days
pub const TOKEN_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("8V7ou9PByAPVdbAetGAPbh3n4WqUVrPgDyzQgDjyTrNC");
pub const SYSVAR_INSTRUCTION: Pubkey =
    Pubkey::from_str_const("Sysvar1nstructions1111111111111111111111111");
pub const MEMO_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    5, 74, 83, 80, 248, 93, 200, 130, 214, 20, 165, 86, 114, 120, 138, 41, 109, 223, 30, 171, 171,
    208, 166, 6, 120, 136, 73, 50, 244, 238, 246, 160,
]);

pub trait ToPubkey {
    fn to_pubkey(&self) -> Pubkey;
}

pub trait ToAddress {
    fn to_address(&self) -> Address;
}

impl ToPubkey for Address {
    fn to_pubkey(&self) -> Pubkey {
        Pubkey::from(self.to_bytes())
    }
}

impl ToAddress for Pubkey {
    fn to_address(&self) -> Address {
        Address::from(self.to_bytes())
    }
}
