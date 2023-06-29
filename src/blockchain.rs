use curl::easy::Easy;
use std::{io::{stdout, Write}, ptr::null};

pub fn get_address_data(search_addr:String) -> Easy {
    let mut easy = Easy::new();
    let mempool_addr_api = String::from("https://mempool.space/api/address/");
    let api_url = mempool_addr_api + &search_addr;
    easy.url(&api_url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    easy
}