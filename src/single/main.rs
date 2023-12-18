extern crate hex;
extern crate alloy_primitives;

use alloy_primitives::{address, b256, hex as ap_hex};
use alloy_primitives::utils::keccak256;
use hex::encode;

fn main() {
    let factory_address = address!("0000000000001b84b1cb32787B0D64758d019317");
    let bytecode = ap_hex!("5859385958601c335a585952fa1582838382515af43d3d93833e601e57fd5bf3");
    let bytecode_hash = keccak256(bytecode);
    println!("Bytecode: {}", encode(bytecode));
    let salt = b256!("0734d56da60852a03e2aafae8a36ffd8c12b32f1926b1dbc6827762a98070000");

    let new_address = factory_address.create2( salt, bytecode_hash);
    println!("New address: {}", encode(new_address));
}