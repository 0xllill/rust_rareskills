# Soroban NFT Contract Security Audit Report


## Critical Findings

### 1. Unauthorized Token Burning

**Description:**  
The `burn` function does not require authorization from the token owner, allowing any address to burn any NFT regardless of ownership.

```rust
fn burn(env: Env, id: i128) {
    // No authorization check present
    env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    let from = read_owner(&env, id);
    write_owner(&env, id, None);
    event::burn(&env, from, id);
}
```

**Impact:**  
This vulnerability permits malicious actors to permanently destroy any NFT in the collection without the owner's consent, leading to irreversible loss of potentially valuable digital assets.

**Recommendation:**  
Implement proper authorization checks in the `burn` function to ensure only the token owner or approved operators can burn tokens:

```rust
fn burn(env: Env, id: i128) {
    env.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    
    // Fetch the token owner
    let from = read_owner(&env, id);
    
    // Require authorization from the token owner
    from.require_auth();
    
    // Execute burn operation
    write_owner(&env, id, None);
    event::burn(&env, from, id);
}
```

### 2. Persistent Approvals After Token Transfer

**Description:**  
The contract fails to clear token-specific approvals when ownership changes. When a token is transferred, any previously set approvals remain active, allowing former approvals to maintain control over tokens even after they change hands.

**Impact:**  
This vulnerability can lead to NFT theft when:
1. A token with an existing approval is transferred to a new owner
2. The previously approved operator can still transfer the token without the new owner's permission
3. Former token owners can regain control of tokens if they receive them back

**Recommendation:**  
Clear all token-specific approvals during any ownership transfer. Modify both `transfer` and `transfer_from` functions to reset approvals:

```rust
// Add to both transfer and transfer_from functions after checking ownership but before changing owner
write_approval(&env, id, None);

// Then proceed with the ownership change
write_owner(&env, id, Some(to.clone()));
```

## Medium Findings

### 3. Administrator Stored in Temporary Storage

**Description:**  
The contract stores the administrator address in temporary storage rather than in persistent storage:

```rust
pub fn write_administrator(env: &Env, id: &Address) {
    let key = DataKey::Admin;
    env.storage().temporary().set(&key, id);
}
```

**Impact:**  
Temporary storage has a limited lifespan and can expire if not regularly extended. If the administrator data expires, the contract may become effectively ownerless, preventing proper administrative functions like minting new tokens or assigning a new admin.

**Recommendation:**  
Store the administrator address in persistent storage to ensure it doesn't expire:

```rust
pub fn write_administrator(env: &Env, id: &Address) {
    let key = DataKey::Admin;
    env.storage().persistent().set(&key, id);
    env.storage().persistent().extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
```

## Low Findings

### 4. Inefficient Ownership Queries

**Description:**  
The `read_all_owned` function iterates through all tokens (0 to supply-1) to find tokens owned by a specific address, which becomes extremely inefficient for large collections.

**Impact:**  
As the collection grows, queries for user-owned tokens will consume more gas and may eventually hit execution limits, degrading user experience and potentially making the function unusable for large collections.

**Recommendation:**  
Implement an index that maps addresses to their owned tokens, updating this index during mint, burn, and transfer operations. This approach provides O(1) lookup time instead of O(n).

### 5. No Approval Validation

**Description:**  
The contract doesn't validate approval operations, allowing redundant or potentially confusing approval states such as owners approving themselves or invalid addresses.

**Impact:**  
While not directly exploitable, these scenarios can lead to confusion, wasted gas, and potential unintended behavior in integrating applications.

**Recommendation:**  
Add validation checks to approval functions:
- Prevent self-approvals (where owner == operator)
- Validate that addresses are well-formed and non-zero
- Consider adding events for approval revocations

### 6. No Checks Against Self-Transfers

**Description:**  
The contract permits transferring tokens to the same address that already owns them, which serves no purpose but still costs gas and generates events.

**Impact:**  
Self-transfers waste computational resources and may confuse tracking systems monitoring transfer events.

**Recommendation:**  
Add a simple check in both transfer functions to prevent self-transfers:

```rust
// Add to both transfer and transfer_from functions
if from == to {
    panic_with_error!(&env, Error::SelfTransferNotAllowed);
}
```

