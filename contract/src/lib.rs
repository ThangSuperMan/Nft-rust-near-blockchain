use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{log, near_bindgen, AccountId, env};

pub struct Token {
    pub id: String,
    pub owner_id: AccountId,
    pub approvals: std::collections::HashMap<String, u64>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FungibleToken {
    user_accounts: LookupMap<AccountId, u128>,
    total_supply: u128,
}

impl Default for FungibleToken {
    fn default() -> Self {
        let mut contract = FungibleToken {
            user_accounts: LookupMap::new(b"m"),
            total_supply: 100000,
        };

        let account_id = env::signer_account_id();
        contract.user_accounts.insert(&account_id, &contract.total_supply);
        log!("account_id is: {}", account_id);
        log!("total_supply is: {}", contract.total_supply);

        let new_total_supply = &contract.total_supply + 100;  
        contract.user_accounts.insert(&account_id, &new_total_supply);

        let thang_account_id: AccountId = AccountId::new_unchecked("dev-123".to_string());
        let thang_total_supply: u128 = 120;

        contract.user_accounts.insert(&thang_account_id, &thang_total_supply);

        return contract
    }
}

#[near_bindgen]
impl FungibleToken {
    pub fn get_total_token(&self) -> u128 {
        self.total_supply.clone()     
    }

    pub fn get_token_account(&self, account_id: AccountId) ->  Option<u128> {
        log!("get_token_account just being executed");
        self.user_accounts.get(&account_id)
    }

    // TODO: References to the FungibleToken struct and we can modify that
    pub fn update_total_supply_by_account_id(&mut self, account_id: AccountId, new_total_supply: u128) -> Option<u128> {
        // TODO: Remove the exists key
        self.user_accounts.remove(&account_id);
            
        // TODO: Create the new account_id with new total supply new value
        self.user_accounts.insert(&account_id, &new_total_supply);

        let result: Option<u128> = self.user_accounts.get(&account_id);

        return result;
    }
}

