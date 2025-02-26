use near_sdk::store::LookupMap;
use near_sdk::{env, near, require, AccountId};

pub type Id = u8;

#[near(contract_state)]
pub struct Contract {
    pub tokens: LookupMap<Id, AccountId>,
    pub approvals: LookupMap<Id, AccountId>,
    pub supply: u16,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, "admin.near".parse().unwrap());
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }
}

// env::predecessor_account_id() == msg.sender

#[near]
impl Contract {

    #[init]
    #[private] // only callable by the contract's account
    pub fn init(
        admin: AccountId
    ) -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, admin);
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }

    pub fn owner_of(&self, id: Id) -> Option<AccountId> {
        self.tokens.get(&id).cloned()
    }


    // FIX-2
        // pub type Id = u16;  // or u32 for even more tokens

        // Then in the mint function:
        // pub fn mint(&mut self) -> Id {
        //     let id = self.supply;  // Use the full u16 value as the ID
        //     self.tokens.insert(id, env::predecessor_account_id());
        //     self.supply += 1;
        //     id
        // }
        
    //E This approach has a limitation - once supply exceeds 255, the first byte will wrap around, potentially causing ID conflicts.
    pub fn mint(&mut self) -> Id {
        //E insert the tokenId and the caller into the tokens map
        self.tokens.insert(
            self.supply.to_le_bytes()[0], //E mint token Id = self.supply  
            env::predecessor_account_id() //E mint token to the caller
        );
        //E @audit once supply exceeds 255 because of the u8 overflow, the first byte will wrap around, potentially causing ID conflicts.
        
        //E increment the supply
        let id = self.supply;
        self.supply += 1;
        id as Id
    }

    pub fn approve(&mut self, id: Id, delegatee: AccountId) {
        //E require the caller to be the owner of the tokenId
        require!(self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id(), "not owner!");
        //E insert the delegatee into the approvals map
        self.approvals.insert(id, delegatee);
    }

    pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(
            //E check if the owner of the tokenId is the predecessor account id
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            
            //E check if the delegatee is the predecessor account id of the approval => @audit if no approval it will cause a runtime error
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );

        //E @audit approvals are never cleared 

        self.tokens.insert(id, receiver);

        // FIX
        //E clear any existing approval for this token
        // if self.approvals.contains_key(&id) {
        //     self.approvals.remove(&id);
        // }

    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};
    use super::*;

    #[test]
    fn exploit_todo() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);
        
    }

    // Test for the ID overflow issue when supply exceeds 255
    #[test]
    fn test_id_overflow() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        
        let mut contract = Contract::default();
        
        // Set supply to 255
        contract.supply = 255;
        
        // Mint token with ID 255
        let token_id_255 = contract.mint();
        assert_eq!(token_id_255, 255);
        assert_eq!(contract.owner_of(255).unwrap(), bob);
        
        // Mint another token, which should have ID 0 due to overflow
        let token_id_overflow = contract.mint();
        
        // This should be 0 due to the .to_le_bytes()[0] conversion
        assert_eq!(token_id_overflow, 0);
        
        // The owner of token 0 should now be bob, not admin
        assert_eq!(contract.owner_of(0).unwrap(), bob);
        
        // Supply should be 257, but the token ID wrapped around to 0
        assert_eq!(contract.supply, 257);
    }
    
    // Test for the approval unwrap issue
    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_transfer_without_approval_unwrap_panic() {
        let admin: AccountId = "admin.near".parse().unwrap();
        let alice: AccountId = "alice.near".parse().unwrap();
        let bob: AccountId = "bob.near".parse().unwrap();
        
        // Admin context
        set_context(admin.clone());
        let mut contract = Contract::default();
        
        // Alice mints a token
        set_context(alice.clone());
        let token_id = contract.mint();
        
        // Bob tries to transfer Alice's token without approval
        set_context(bob.clone());
        // This should panic because there's no approval for Bob
        contract.transfer(token_id, bob.clone());
    }
    
    // Test for the missing approval clearing after transfer
    #[test]
    fn test_approval_not_cleared_after_transfer() {
        let alice: AccountId = "alice.near".parse().unwrap();
        let bob: AccountId = "bob.near".parse().unwrap();
        let charlie: AccountId = "charlie.near".parse().unwrap();
        
        // Set up Alice as the context
        set_context(alice.clone());
        let mut contract = Contract::default();
        
        // Alice mints a token
        let token_id = contract.mint();
        assert_eq!(contract.owner_of(token_id).unwrap(), alice);
        
        // Alice approves Bob to transfer her token
        contract.approve(token_id, bob.clone());
        
        // Bob transfers Alice's token to Charlie
        set_context(bob.clone());
        contract.transfer(token_id, charlie.clone());
        
        // Charlie now owns the token
        assert_eq!(contract.owner_of(token_id).unwrap(), charlie);
        
        // Bob should not be able to transfer the token again, but the current implementation
        // doesn't clear approvals, so Bob can still transfer Charlie's token
        set_context(bob.clone());
        contract.transfer(token_id, bob.clone());
        
        // Bob now owns the token, which shouldn't be possible
        assert_eq!(contract.owner_of(token_id).unwrap(), bob);
    }
    
    // Test for generic error handling with unwrap()
    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_approve_nonexistent_token() {
        let alice: AccountId = "alice.near".parse().unwrap();
        let bob: AccountId = "bob.near".parse().unwrap();
        
        set_context(alice.clone());
        let mut contract = Contract::default();
        
        // Try to approve a token that doesn't exist
        // This will panic with a generic error rather than a clear message
        contract.approve(99, bob.clone());
    }
    
    // Test the consistency between supply value and returned ID
    #[test]
    fn test_supply_and_id_consistency() {
        let alice: AccountId = "alice.near".parse().unwrap();
        
        set_context(alice.clone());
        let mut contract = Contract::default();
        
        // Set supply to 256 to demonstrate the inconsistency
        contract.supply = 256;
        
        // Mint a token
        let token_id = contract.mint();
        
        // The function returns the pre-increment supply as ID (cast to u8)
        // But internally it uses the to_le_bytes()[0] value for storage
        assert_eq!(token_id, 0); // Because 256 as u8 = 0
        
        // Check what was actually stored
        let owner = contract.owner_of(0).unwrap();
        assert_eq!(owner, alice);
        
        // But the actual value returned was contract.supply (before increment) cast to Id
        assert_eq!(contract.supply, 257);
    }
    
    // Test multiple mints by same account
    #[test]
    fn test_multiple_mints() {
        let alice: AccountId = "alice.near".parse().unwrap();
        
        set_context(alice.clone());
        let mut contract = Contract::default();
        
        // Mint multiple tokens
        let id1 = contract.mint();
        let id2 = contract.mint();
        let id3 = contract.mint();
        
        // Verify all tokens are owned by Alice
        assert_eq!(contract.owner_of(id1).unwrap(), alice);
        assert_eq!(contract.owner_of(id2).unwrap(), alice);
        assert_eq!(contract.owner_of(id3).unwrap(), alice);
        
        // Verify IDs are sequential
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }
    
    // Auxiliar fn: create a mock context
    fn set_context(predecessor: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        
        testing_env!(builder.build());
    }

}
