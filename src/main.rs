extern crate hex;
extern crate alloy_primitives;

use alloy_primitives::utils::keccak256;
use hex::encode;

fn main() {
    let msg = "abcd";
    // convert msg to utf8 bytes
    let data = msg.as_bytes();
    
    // call keccak256
    let hash = keccak256(data);

    // print
    println!("Hash of {} is {}", msg, encode(hash));
    
    let mut counter = 0;
    let mut num_digits = 0;

    // start timer
    let start = std::time::Instant::now();

    while num_digits < 4 {
        let msg = format!("{}{}", "abcd", counter);
        let data = msg.as_bytes();
        let hash = keccak256(data);
        let hash_str = encode(hash);
        if hash_str.starts_with("0000") {
            println!("Hash of {} is {}", msg, hash_str);
            num_digits += 1;
            // print hash rate
            let duration = start.elapsed();
            let secs = duration.as_secs();
            
            let rate = counter as f64 / secs as f64;
            println!("Hash rate: {} hashes per second", rate);
        }
        counter += 1;
    }
}