use curl::easy::Easy;
use std::io::{stdout, Write};

pub fn get_address_data(pubkey:String) -> Easy {
    let mut easy = Easy::new();
    easy.url("https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    return easy;
}