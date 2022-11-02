use crate::secp256k1::Secp256k1KeyPair;
use fastcrypto::traits::KeyPair;
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

const FIXED_BYTES_NUM: usize = 3;

pub fn pubkeymine() {
    let mut count: u64 = rand::random();

    let now = Instant::now();

    loop {
        count += 1;

        let mut rng = StdRng::seed_from_u64(count);

        let newkey = Secp256k1KeyPair::generate(&mut rng);

        let newbytes = newkey.public().pubkey.serialize();

        if newbytes[1..=FIXED_BYTES_NUM] == vec![100; FIXED_BYTES_NUM] {
            println!("{:?}", newbytes);
            println!("{:?}", &newbytes[1..3]);
            break;
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
