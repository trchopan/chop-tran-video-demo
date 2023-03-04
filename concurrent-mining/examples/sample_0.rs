use std::time::Instant;

use concurrent_mining::{hash_block, load_block, print_progress};

const SAMPLE_FILE: &str = "./sample_block.yaml";
const TARGET_HASH_COUNT: u32 = 5_000_000;

fn main() {
    let block = load_block(SAMPLE_FILE.try_into().unwrap());

    let now = Instant::now();

    for nonce in 0..TARGET_HASH_COUNT {
        print_progress(nonce, TARGET_HASH_COUNT);

        let hash = hash_block(&nonce, &block);
        if hash.starts_with("00000") {
            println!("Nonce: {nonce} - Hash: {hash}");
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
