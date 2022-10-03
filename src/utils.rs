use std::collections::HashMap;
use fastcrypto::hash::{Blake3, HashFunction};
use crate::traits::ByteStatisticsSummator;

pub fn summator_statistics(msg: &str, retries: u32, summator: &dyn ByteStatisticsSummator, hashmap: &mut HashMap<u32, u32>) {
    let msg_digest = Blake3::digest(msg.as_bytes());
    for i in 0..retries {
        let msg_digest_plus_i = [msg_digest.as_ref(), &i.to_le_bytes()].concat();
        let final_digest = Blake3::digest(&msg_digest_plus_i);
        let sum = summator.sum(&final_digest.as_ref());
        hashmap.insert(sum, **hashmap.get(&sum).get_or_insert(&0) + 1);
    }
}