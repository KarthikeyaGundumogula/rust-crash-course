use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut hash_map:HashMap<String, u32>  = HashMap::new();
    hash_map.insert(address,amount);
    hash_map
}
