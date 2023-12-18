extern crate hex;
extern crate alloy_primitives;

use alloy_primitives::{Address, FixedBytes, Bytes};
use alloy_primitives::{address, b256, bytes, hex as ap_hex};
use alloy_primitives::utils::keccak256;
use rand::Rng;
use hex::encode;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let solver_address:Address = address!("819Caa13f9b5211167Ef696aA7dDadd9EA3bb1EB");
    
    // https://github.com/Arachnid/deterministic-deployment-proxy/tree/master

    // Nick's Deterministic Deployment Proxy
    let factory_address:Address = address!("4e59b44847b379578588920ca78fbf26c0b4956c");
    
    // Original ERCRefDeploy address
    // let factory_address:Address = address!("C8fF050202f59acf1c3D63CdC1ae400f2aA4ae3a");
    
    // get bytes from hex string
    let bytecode = ap_hex!("608060405234801561001057600080fd5b50610302806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c8063a425c82114610030575b600080fd5b61004a60048036038101906100459190610214565b610060565b60405161005791906102b1565b60405180910390f35b600080828451602086016000f59050803b61007a57600080fd5b8091505092915050565b6000604051905090565b600080fd5b600080fd5b600080fd5b600080fd5b6000601f19601f8301169050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6100eb826100a2565b810181811067ffffffffffffffff8211171561010a576101096100b3565b5b80604052505050565b600061011d610084565b905061012982826100e2565b919050565b600067ffffffffffffffff821115610149576101486100b3565b5b610152826100a2565b9050602081019050919050565b82818337600083830152505050565b600061018161017c8461012e565b610113565b90508281526020810184848401111561019d5761019c61009d565b5b6101a884828561015f565b509392505050565b600082601f8301126101c5576101c4610098565b5b81356101d584826020860161016e565b91505092915050565b6000819050919050565b6101f1816101de565b81146101fc57600080fd5b50565b60008135905061020e816101e8565b92915050565b6000806040838503121561022b5761022a61008e565b5b600083013567ffffffffffffffff81111561024957610248610093565b5b610255858286016101b0565b9250506020610266858286016101ff565b9150509250929050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b600061029b82610270565b9050919050565b6102ab81610290565b82525050565b60006020820190506102c660008301846102a2565b9291505056fea26469706673582212203fb804f910d091ef41072e615dd46be1fbe316dfaf7a6641bd2d8e44393ec96764736f6c63430008110033");

    let bytecode_hash:FixedBytes<32> = keccak256(bytecode);
    println!("Bytecode: {}", encode(bytecode));
    println!("Bytecode hash: {}", encode(bytecode_hash));


    // start timer
    let start = std::time::Instant::now();

    let mut handles = vec![];
    // let sharedCounter = Arc::new(Mutex::new(0));

    let shared_counter:Arc<Mutex<i64>> = Arc::new(Mutex::new(0));
    let shared_current_best_address: Arc<Mutex<Address>> = Arc::new(Mutex::new(address!("ffffffffffffffffffffffffffffffffffffffff")));
    
    for thread_index in 0..8 {

    let shared_counter_clone = Arc::clone(&shared_counter);
    let shared_current_best_address_clone = Arc::clone(&shared_current_best_address);
        let handle = thread::spawn(move || {
            let mut local_counter = 0;
            loop {
                // let data = random data to be input into keccak256
                // generate random number
                let mut rng = rand::thread_rng();     
                let new_source_salt: [u8; 32] = rng.gen();
                
                let solver_address_str = encode(solver_address);
                let new_source_salt_str = encode(new_source_salt);
                // keccak256 of concatenation of solverAddress and new_source_salt
                let new_salt = keccak256(&[solver_address_str, new_source_salt_str].concat());
                let new_address = factory_address.create2( new_salt, bytecode_hash);
                let new_address_str = encode(new_address);
                if *shared_current_best_address_clone.lock().unwrap() > new_address {
                    // update shared current best address
                    let mut shared_current_best_address = shared_current_best_address_clone.lock().unwrap();
                    *shared_current_best_address = new_address;
                    println!("Thread {} hereby reports", thread_index);
                    println!("Source salt: {}", encode(new_source_salt));
                    println!("New salt: {}", encode(new_salt));
                    println!("New address: {}", new_address_str);
                    println!("Solver address: {}", encode(solver_address));
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