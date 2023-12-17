extern crate hex;
extern crate alloy_primitives;

use alloy_primitives::{Address, FixedBytes};
use alloy_primitives::{address, b256};
use alloy_primitives::utils::keccak256;
use rand::Rng;
use hex::encode;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const SOLVER_ADDRESS:Address = address!("819Caa13f9b5211167Ef696aA7dDadd9EA3bb1EB");
    const FACTORY_ADDRESS:Address = address!("5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f");
    const BYTECODE_HASH:FixedBytes<32> = b256!("78c28f67559fd19647ff0f6c6ec627527c328036073f0a0e0a791dc5f90506b1"); 
    
    let num_digits = 4;

    // start timer
    let start = std::time::Instant::now();

    let found = Arc::new(Mutex::new(false));

    let mut handles = vec![];
    // let sharedCounter = Arc::new(Mutex::new(0));

    let shared_counter:Arc<Mutex<i64>> = Arc::new(Mutex::new(0));
    let shared_current_best_address: Arc<Mutex<Address>> = Arc::new(Mutex::new(address!("ffffffffffffffffffffffffffffffffffffffff")));
    
    for thread_index in 0..36 {

    let shared_counter_clone = Arc::clone(&shared_counter);
    let shared_current_best_address_clone = Arc::clone(&shared_current_best_address);
        let handle = thread::spawn(move || {
            let mut local_counter = 0;
            let mut found_local = 0;
            loop {
                // let data = random data to be input into keccak256
                // generate random number
                let mut rng = rand::thread_rng();
                let mut data = vec![0u8; num_digits];        
                let new_source_salt: [u8; 32] = rng.gen();
                
                let solver_address_str = encode(SOLVER_ADDRESS);
                let new_source_salt_str = encode(new_source_salt);
                // keccak256 of concatenation of solverAddress and new_source_salt
                let new_salt = keccak256(&[solver_address_str, new_source_salt_str].concat());
                let new_address = FACTORY_ADDRESS.create2( new_salt, BYTECODE_HASH);
                let new_address_str = encode(new_address);
                if *shared_current_best_address_clone.lock().unwrap() > new_address {
                    // update shared current best address
                    let mut shared_current_best_address = shared_current_best_address_clone.lock().unwrap();
                    *shared_current_best_address = new_address;
                    println!("Thread: {} reports new best address: {}", thread_index, new_address_str);
                }
                local_counter += 1;

                if local_counter % (65536 *2) == 0 {
                    // merge shared counter and clear local counter
                    let mut shared_counter = shared_counter_clone.lock().unwrap();
                    *shared_counter += local_counter;
                    local_counter = 0;
                    let duration: std::time::Duration = start.elapsed();
                    let micros = duration.as_micros();
                    // get primitive value of shared counter
                    let shared_counter_value:i64 = *shared_counter;
                    let rate = (shared_counter_value as f64 / micros as f64) * 1000000.0;
                    println!("Thread: {} reports Hash rate: {} hashes per second, current count {}", thread_index, rate, shared_counter_value);
                
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}