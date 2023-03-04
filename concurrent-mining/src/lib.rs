use std::{fs, path::PathBuf};

use sha2::{Digest, Sha256};

pub fn load_block(fp: PathBuf) -> String {
    fs::read_to_string(fp).expect("Something went wrong reading the file")
}

pub fn hash_block(nonce: &u32, block: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update((nonce.to_string() + block).as_bytes());
    hex::encode(hasher.finalize())
}

pub fn print_progress(nonce: u32, max: u32) {
    let tenth_percent = max / 10;
    if nonce % tenth_percent == 0 {
        let done_percent = 100 * nonce / max;
        println!("Done {done_percent} %");
    }
}
