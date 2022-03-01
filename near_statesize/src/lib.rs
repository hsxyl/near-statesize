mod macro_rules;

use std::collections::HashMap;
use near_sdk::collections::{TreeMap, UnorderedMap, UnorderedSet, Vector};
pub use near_statesize_derive::*;

pub trait NearStateSize {
    /// sdf
    fn state_size(&self) -> usize;
}

//
// In a real version of this library there would be lots more impls here, but
// here are some interesting ones.
//

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

impl_near_statesize_iter!(Vec);
impl_near_statesize_iter!(UnorderedSet);
impl_near_statesize_iter_key_value!(HashMap);
impl_near_statesize_iter_key_value!(UnorderedMap);
impl_near_statesize_iter_key_value!(TreeMap);

impl <T> NearStateSize for Vector<T> {
    fn state_size(&self) -> usize {
        self.iter_raw().map(|e|e.len()).sum()
    }
}
