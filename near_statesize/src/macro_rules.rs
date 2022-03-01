#[macro_export]
macro_rules! impl_near_statesize {
    ($ty: tt, $size: literal) => {
        impl NearStateSize for $ty {
            fn state_size(&self) -> usize {
               $size
            }
        }
    }
}

#[macro_export]
macro_rules! impl_near_statesize_iter{
    ($ty: tt, $size: literal) => {
        impl <T>NearStateSize for $ty<T> where T: NearStateSize+BorshSerialize+BorshDeserialize  {
            fn state_size(&self) -> usize {
                self.iter().map(|e|e.state_size()).sum::<usize>() + $size
            }
        }
    }
}

#[macro_export]
macro_rules! impl_near_statesize_iter_key_value {
    ($ty: tt, $size: literal) => {
        impl <K,V>NearStateSize for $ty<K,V>
            where
                K: NearStateSize+BorshSerialize+BorshDeserialize,
                V: NearStateSize+BorshSerialize+BorshDeserialize {
            fn state_size(&self) -> usize {
                self.iter().map(|(k,v)|k.state_size()+v.state_size()).sum::<usize>() + $size
            }
        }
    }
}
