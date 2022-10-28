use fastcrypto::traits::KeyPair;
use rand::{rngs::StdRng, SeedableRng};
use crate::secp256k1::Secp256k1KeyPair;

pub fn pubkeymine() {

    let mut count: u64= rand::random();

  loop {
    count += 1;

    let mut rng = StdRng::seed_from_u64(count);

    let newkey = Secp256k1KeyPair::generate(&mut rng);
    
    let newbytes = newkey.public().pubkey.serialize();
    
    let fixedbytes = newbytes[1];

    if fixedbytes == 100 {
      println!("{}", count);
      break;
    }
  }


}