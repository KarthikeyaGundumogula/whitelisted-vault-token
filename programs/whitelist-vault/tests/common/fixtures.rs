use solana_sdk::native_token::LAMPORTS_PER_SOL;

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
