use std::env;

use concurrent_mining::{hash_block, load_block};

const SAMPLE_FILE: &str = "./sample_block.yaml";

fn main() {
    let args: Vec<String> = env::args().collect();
    let block = load_block(SAMPLE_FILE.try_into().unwrap());

    let nonce: u32 = args
        .get(1)
        .map(|nonce_str| {
            nonce_str
                .parse::<u32>()
                .expect("cannot parse nonce from string")
        })
        .expect("cannot get nonce from first args");

    let hash = hash_block(&nonce, &block);
    println!("Nonce: {nonce} - Hash: {hash}");
}
