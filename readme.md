# Vault Deposit Method with Transfer Hooks

## The Problem

Tokens with **transfer hooks** cause re-entrancy errors when using traditional CPI transfers:

```
Vault Program → Token Program → Transfer Hook → Vault Program ❌ RE-ENTRANCY
```

## Our Solution

**Client-side transfer + On-chain verification using memos**

### How It Works

1. **User transfers tokens directly** (client-side) with a memo
2. **Vault program verifies** the memo matches expected format
3. **No CPI transfer** = No re-entrancy

### Transaction Structure

```
┌─────────────────────────────────────┐
│ [0] SPL Memo Program                │  ← "deposit:user:1000:nonce"
│ [1] Token Transfer (with hook)      │  ← User → Vault transfer
│ [2] Vault Deposit Instruction       │  ← Reads memo, records deposit
└─────────────────────────────────────┘
```

### Memo Format

```
deposit:{user_pubkey}:{amount}:{nonce}
```

### Security Features

✅ **Atomicity**: All 3 instructions succeed or all fail (no stuck tokens)  
✅ **Memo Transfer Extension**: Vault ATA rejects transfers without memo  
✅ **Instruction Introspection**: Program reads & verifies memo content  
✅ **No Re-entrancy**: No CPI to token program  

### Key Code

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64, nonce: u64) -> Result<()> {
    // 1. Build expected memo
    let expected_memo = format!("deposit:{}:{}:{}", ctx.accounts.user.key(), amount, nonce);
    
    // 2. Verify memo exists in transaction (via instructions sysvar)
    verify_memo_in_transaction(&ctx.accounts.instructions, &expected_memo)?;
    
    // 3. Update balance
    ctx.accounts.user_vault.balance += amount;
    
    Ok(())
}
```

**That's it.** Transfer happens client-side, verification happens on-chain, no re-entrancy.