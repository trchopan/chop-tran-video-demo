use concurrent_mining::{load_block, hash_block};

const SAMPLE_FILE: &str = "./sample_block.yaml";

fn main() {
    let block = load_block(SAMPLE_FILE.try_into().unwrap());
    let mut nonce = 0;

    loop {
        let hash = hash_block(&nonce, &block);
        if hash.starts_with("00000") {
            println!("Nonce: {nonce} - Hash: {hash}");
            break;
        }
        nonce += 1;
    }
}
