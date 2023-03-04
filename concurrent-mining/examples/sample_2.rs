use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use concurrent_mining::{hash_block, load_block, print_progress};

const SAMPLE_FILE: &str = "./sample_block.yaml";
const TARGET_HASH_COUNT: u32 = 5_000_000;
const THREAD_AMOUNT: usize = 10;

fn main() {
    let block = load_block(SAMPLE_FILE.try_into().unwrap());

    let nonce_count = Arc::new(Mutex::new(0u32));

    let spawn_worker = |nonces: Vec<u32>| {
        let block = block.clone();
        let nonce_count = Arc::clone(&nonce_count);
        thread::spawn(move || {
            for nonce in nonces {
                let hash = hash_block(&nonce, &block);

                if hash.starts_with("00000") {
                    println!("Nonce: {nonce} - Hash: {hash}");
                }

                let mut nonce_count_inner = nonce_count.lock().unwrap();
                print_progress(*nonce_count_inner, TARGET_HASH_COUNT);
                *nonce_count_inner += 1;
            }
        })
    };

    let now = Instant::now();

    let all_nonces = (0..TARGET_HASH_COUNT).collect::<Vec<u32>>();
    let nonce_chunks = all_nonces.chunks(TARGET_HASH_COUNT as usize / THREAD_AMOUNT);

    let mut handles = vec![];
    nonce_chunks.for_each(|nonce_chunk| {
        handles.push(spawn_worker(nonce_chunk.to_vec()));
    });

    // Join workers with main thread
    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
