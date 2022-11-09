#[macro_use]
extern crate criterion;

mod mining_benches {
    use super::*;
    use criterion::*;
    use fastcrypto::hash::{Blake3, HashFunction, Sha256, Sha3_256};
    use fastcrypto::{ed25519::Ed25519KeyPair, secp256k1::Secp256k1KeyPair, traits::KeyPair};
    use rand::{prelude::ThreadRng, thread_rng};
    use signature::Signer;

    fn key_generation(c: &mut Criterion) {
        // Note that for 3 bytes, benchmarks are very slow.
        const FIXED_BYTES_NUM: [usize; 3] = [1, 2, 3];

        for num in FIXED_BYTES_NUM {
            let mut csprng1: ThreadRng = thread_rng();
            let mut csprng2 = csprng1.clone();

            c.bench_function(
                &("Ed25519_keypair_gen_zerobytes=".to_owned() + &num.to_string()),
                move |b| {
                    b.iter(|| loop {
                        if Ed25519KeyPair::generate(&mut csprng1).public().as_ref()[1..=num]
                            == vec![0u8; num]
                        {
                            break;
                        }
                    })
                },
            );

            c.bench_function(
                &("ECDSA_secp256k1_keypair_gen_zerobytes=".to_owned() + &num.to_string()),
                move |b| {
                    b.iter(|| loop {
                        if Secp256k1KeyPair::generate(&mut csprng2).public().as_ref()[1..=num]
                            == vec![0u8; num]
                        {
                            break;
                        }
                    })
                },
            );
        }
    }

    fn signing(c: &mut Criterion) {
        // Note that for 3 bytes, benchmarks are very slow.
        const FIXED_BYTES_NUM: [usize; 3] = [1, 2, 3];

        for num in FIXED_BYTES_NUM {
            let mut csprng1: ThreadRng = thread_rng();
            let mut csprng2 = csprng1.clone();
            let ed25519_keypair = Ed25519KeyPair::generate(&mut csprng1);
            let ecdsa_keypair = Secp256k1KeyPair::generate(&mut csprng2);
            let mut counter = 0u64;

            c.bench_function(
                &("Ed25519_sign_zerobytes=".to_owned() + &num.to_string()),
                move |b| {
                    b.iter(|| loop {
                        // Note that ed25519 signing is deterministic, thus we retry with a counter.
                        counter = rand::random();
                        if ed25519_keypair
                            .try_sign(&counter.to_le_bytes())
                            .unwrap()
                            .as_ref()[1..=num]
                            == vec![0u8; num]
                        {
                            break;
                        }
                    })
                },
            );

            c.bench_function(
                &("ECDSA_secp256k1_sign_zerobytes=".to_owned() + &num.to_string()),
                move |b| {
                    b.iter(|| loop {
                        // Note that fastcrypto's ecdsa signing is deterministic, thus we retry
                        // with a counter.
                        counter = rand::random();
                        if ecdsa_keypair
                            .try_sign(&counter.to_le_bytes())
                            .unwrap()
                            .as_ref()[1..=num]
                            == vec![0u8; num]
                        {
                            break;
                        }
                    })
                },
            );
        }
    }

    fn hashing(c: &mut Criterion) {
        // Note that for 3 bytes, benchmarks are very slow.
        const FIXED_BYTES_NUM: [usize; 3] = [1, 2, 3];

        for num in FIXED_BYTES_NUM {
            let mut counter = 0u64;

            c.bench_function(
                &("sha2_256_zerobytes=".to_owned() + &num.to_string()),
                move |b| {
                    b.iter(|| loop {
                        // Note that ed25519 signing is deterministic, thus we retry with a counter.
                        counter = rand::random();
                        if Sha256::digest(&counter.to_le_bytes()).as_ref()[..num] == vec![0u8; num]
                        {
                            break;
                        }
                    })
                },
            );

            c.bench_function(
                &("sha3_256_zerobytes=".to_owned() + &num.to_string()),
                move |b| {
                    b.iter(|| loop {
                        // Note that ed25519 signing is deterministic, thus we retry with a counter.
                        counter = rand::random();
                        if Sha3_256::digest(&counter.to_le_bytes()).as_ref()[..num]
                            == vec![0u8; num]
                        {
                            break;
                        }
                    })
                },
            );

            c.bench_function(
                &("blake3_256_zerobytes=".to_owned() + &num.to_string()),
                move |b| {
                    b.iter(|| loop {
                        // Note that ed25519 signing is deterministic, thus we retry with a counter.
                        counter = rand::random();
                        if Blake3::digest(&counter.to_le_bytes()).as_ref()[..num] == vec![0u8; num]
                        {
                            break;
                        }
                    })
                },
            );
        }
    }

    criterion_group! {
        name = mining_benches;
        config = Criterion::default().significance_level(0.1).sample_size(10);
        targets =
            key_generation,
            signing,
            hashing,
    }
}

criterion_main!(mining_benches::mining_benches,);
