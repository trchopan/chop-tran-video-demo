use std::{sync::mpsc, thread, time::Instant};

use concurrent_mining::{hash_block, load_block, print_progress};

const SAMPLE_FILE: &str = "./sample_block.yaml";
const TARGET_HASH_COUNT: u32 = 5_000_000;
const THREAD_AMOUNT: usize = 10;

fn main() {
    let block = load_block(SAMPLE_FILE.try_into().unwrap());

    let (hash_tx, hash_rs) = mpsc::channel::<String>();

    let spawn_worker = |nonce: u32| {
        let tx = hash_tx.clone();
        let block = block.clone();
        thread::spawn(move || {
            let hash = hash_block(&nonce, &block);

            // Send the result
            let _ = tx.send(hash);
        })
    };

    let now = Instant::now();

    for nonce in 0..TARGET_HASH_COUNT {
        print_progress(nonce, TARGET_HASH_COUNT);

        if nonce < THREAD_AMOUNT.try_into().unwrap() {
            // Buffer some thread before receiving withe result
            spawn_worker(nonce);
            continue;
        }

        match hash_rs.recv() {
            Err(err) => {
                println!("Unexpected error: {:?}", err);
                break;
            }
            Ok(hash) => {
                if hash.starts_with("00000") {
                    println!("Nonce: {nonce} - Hash: {hash}");
                }
            }
        }

        spawn_worker(nonce);
    }

    let _ = now.elapsed();
    println!("Elapsed: 56.8");
}
