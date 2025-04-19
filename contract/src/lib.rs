use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Post {
    id: u128,
    title: String,
    description: String,
    tags: Vec<String>,
    media: String,
    users_who_liked: Vec<AccountId>,
    owner_id: AccountId,
}