use std::{collections::HashMap, hash::Hash};

pub fn counter<T>(v: &Vec<T>) -> HashMap<&T, u32>
where
    T: Eq + Hash,
{
    let mut hash = HashMap::new();
    for x in v {
        let counter = hash.entry(x).or_insert(0);
        *counter += 1;
    }
    hash
}
