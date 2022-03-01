mod macro_rules;

use std::collections::HashMap;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap, UnorderedMap, UnorderedSet, Vector};
pub use near_statesize_derive::*;

pub trait NearStateSize {
    fn state_size(&self) -> usize;
}

impl_near_statesize!(u8,1);
impl_near_statesize!(u32,4);
impl_near_statesize!(u64,8);
impl_near_statesize!(u128,16);
impl_near_statesize!(i8,1);
impl_near_statesize!(i32,4);
impl_near_statesize!(i64,8);
impl_near_statesize!(i128,16);

impl NearStateSize for String {
    fn state_size(&self) -> usize {
        self.len()
    }
}

impl_near_statesize_iter!(Vec,4);
impl_near_statesize_iter!(UnorderedSet,4);
impl_near_statesize_iter_key_value!(HashMap,4);
impl_near_statesize_iter_key_value!(UnorderedMap,4);

impl <T> NearStateSize for Vector<T> {
    fn state_size(&self) -> usize {
        self.iter_raw().map(|e|e.len()).sum()
    }
}

impl<K, V> NearStateSize for TreeMap<K, V>
    where
        K: NearStateSize+Ord + Clone + BorshSerialize + BorshDeserialize,
        V: NearStateSize+BorshSerialize + BorshDeserialize,
{
    fn state_size(&self) -> usize {
        self.iter().map(|(k,v)|k.state_size()+v.state_size()).sum()
    }
}