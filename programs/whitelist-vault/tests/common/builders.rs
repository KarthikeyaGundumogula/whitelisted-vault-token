// use solana_sdk::pubkey::{ Pubkey};

// use super::{fixtures::*, setup::Setup};


// const TOKEN_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
// const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

// /// Builder for Make instruction
// pub struct DepositInstructionBuilder<'a> {
//     setup: &'a Setup,
//     deposit: u64,
//     receive: u64,
//     seed: u64,
// }

// impl<'a> DepositInstructionBuilder<'a> {
//     pub fn new(setup: &'a Setup) -> Self {
//         Self {
//             setup,
//             deposit: DEPOSIT_AMOUNT,
//             receive: RECEIVE_AMOUNT,
//             seed: SEED,
//         }
//     }

//     pub fn deposit(mut self, amount: u64) -> Self {
//         self.deposit = amount;
//         self
//     }

//     pub fn receive(mut self, amount: u64) -> Self {
//         self.receive = amount;
//         self
//     }

//     pub fn seed(mut self, seed: u64) -> Self {
//         self.seed = seed;
//         self
//     }

//     pub fn build(self) -> Instruction {
//         Instruction {
//             program_id: PROGRAM_ID,
//             accounts: crate::accounts::Make {
//                 maker: self.setup.maker,
//                 mint_a: self.setup.mint_a,
//                 mint_b: self.setup.mint_b,
//                 maker_ata_a: self.setup.maker_ata_a,
//                 escrow: self.setup.escrow,
//                 vault: self.setup.vault,
//                 associated_token_program: ASSOCIATED_PROGRAM_ID,
//                 token_program: TOKEN_PROGRAM_ID,
//                 system_program: SYSTEM_PROGRAM_ID,
//             }
//             .to_account_metas(None),
//             data: crate::instruction::Make {
//                 deposit: self.deposit,
//                 seed: self.seed,
//                 receive: self.receive,
//             }
//             .data(),
//         }
//     }
// }

// /// Builder for Take instruction
// pub struct TakeInstructionBuilder<'a> {
//     setup: &'a Setup,
// }

// impl<'a> TakeInstructionBuilder<'a> {
//     pub fn new(setup: &'a Setup) -> Self {
//         Self { setup }
//     }

//     pub fn build(self) -> Instruction {
//         Instruction {
//             program_id: PROGRAM_ID,
//             accounts: crate::accounts::Take {
//                 taker: self.setup.taker,
//                 maker: self.setup.maker,
//                 mint_a: self.setup.mint_a,
//                 mint_b: self.setup.mint_b,
//                 taker_ata_a: self.setup.taker_ata_a,
//                 taker_ata_b: self.setup.taker_ata_b,
//                 maker_ata_b: self.setup.maker_ata_b,
//                 escrow: self.setup.escrow,
//                 vault: self.setup.vault,
//                 associated_token_program: ASSOCIATED_PROGRAM_ID,
//                 token_program: TOKEN_PROGRAM_ID,
//                 system_program: SYSTEM_PROGRAM_ID,
//             }
//             .to_account_metas(None),
//             data: crate::instruction::Take {}.data(),
//         }
//     }
// }

// /// Builder for Refund instruction
// pub struct RefundInstructionBuilder<'a> {
//     setup: &'a Setup,
// }

// impl<'a> RefundInstructionBuilder<'a> {
//     pub fn new(setup: &'a Setup) -> Self {
//         Self { setup }
//     }

//     pub fn build(self) -> Instruction {
//         Instruction {
//             program_id: PROGRAM_ID,
//             accounts: crate::accounts::Refund {
//                 maker: self.setup.maker,
//                 mint_a: self.setup.mint_a,
//                 maker_ata_a: self.setup.maker_ata_a,
//                 escrow: self.setup.escrow,
//                 vault: self.setup.vault,
//                 token_program: TOKEN_PROGRAM_ID,
//                 system_program: SYSTEM_PROGRAM_ID,
//             }
//             .to_account_metas(None),
//             data: crate::instruction::Refund {}.data(),
//         }
//     }
// }