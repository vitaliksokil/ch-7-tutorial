use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, near_bindgen, serde::{Deserialize, Serialize}, PanicOnDefault, env, Promise};
use near_sdk::collections::UnorderedMap;


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum FundraiserPurpose{
    Medicine,
    Children,
    Disability,
    Environment,
    Animal,
    Education
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Fundraiser{
    owner_id: AccountId,
    title: String,
    description: String,
    banner_image: String,
    // in yoctoNear , 1n = 10^24
    total_donated: u128,
    fundraising_amount: u128,
    fundraising_purpose: FundraiserPurpose
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub fundraisers: UnorderedMap<u8,Fundraiser>,
    pub total_fundraisers: u8
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            fundraisers: UnorderedMap::new(b"d".to_vec()),
            total_fundraisers: 0
        }
    }

    pub fn get_all_fundraisers(&self) -> HashMap<u8,Fundraiser>{
        self.fundraisers.iter().collect()
    }
    pub fn get_fundraiser_by_id(&self, id: u8) -> Fundraiser{
        self.fundraisers.get(&id).expect("Fundraiser doesn't exist")
    }

    #[payable]
    pub fn donate(&mut self, fundraiser_id: u8){
        let deposit: u128 = env::attached_deposit();
        let mut fundraiser: Fundraiser = self.fundraisers.get(&fundraiser_id).expect("Fundraiser doesn't exist");

        fundraiser.total_donated += deposit;
        self.fundraisers.remove(&fundraiser_id);
        self.fundraisers.insert(&fundraiser_id, &fundraiser);

        Promise::new(fundraiser.owner_id).transfer(deposit);
    }

    pub fn add_new_fundraiser(
        &mut self,
        title: String,
        description: String,
        banner_image: String,
        fundraising_amount: String,
        fundraising_purpose: FundraiserPurpose
    ){
       let fundraising_amount_u128 = fundraising_amount.parse::<u128>().unwrap();

        assert!(title != "", "Abort. Title is empty!");
        assert!(description != "", "Abort. Description is empty!");
        assert!(banner_image != "", "Abort. Banner image is empty!");
        assert!(fundraising_amount_u128 != 0, "Abort.Fundraiser amount cannot be zero!");

        self.total_fundraisers += 1;
        let id:u8 = self.total_fundraisers;
        let owner_account_id: AccountId = env::predecessor_account_id();

        self.fundraisers.insert(
            &id,
            &Fundraiser{
                owner_id: owner_account_id,
                title,
                description,
                banner_image,
                total_donated: 0,
                fundraising_amount: fundraising_amount_u128,
                fundraising_purpose
            }
        );
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
