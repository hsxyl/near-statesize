use std::env::Args;
use std::sync::Arc;
use near_sdk::collections::UnorderedSet;
use near_statesize::NearStateSize;
use near_sdk::{BorshStorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Test
}

#[derive(NearStateSize)]
struct Test {
    a: String,
    b: u64,
    c: UnorderedSet<String>
}

fn main() {
    let x = Test {
        a: "".to_string(),
        b: 0,
        c: UnorderedSet::new(StorageKey::Test)
    };
    println!("{}",x.state_size());
}
