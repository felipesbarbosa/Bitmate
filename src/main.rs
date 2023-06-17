//Will implement later:
/*
use bitcoin::bip32::{ChildNumber, ExtendedPubKey};
use bitcoin::taproot::{tapscript::TapScript, TaprootScript};
*/
use bitcoin::network::constants::Network;
use std::io;

mod address;
mod blockchain;
use address::{create_new_address};
use blockchain::{get_address_data};

fn main() {
    //Chosing the desired bitcoin network 
    let network = Network::Testnet;
    let mut user_input_str = String::new();
    let mut user_input:i32 = 5;
    println!("Welcome to freedom!\nThis is an app in development for using bitcoin");
    println!("");
    println!("Choose a option bellow:");
    println!("1 - Select network (Default: Testnet)");
    println!("2 - Create new address");
    println!("3 - Read bitcoin address info");
    println!("0 - Exit");

    while user_input != 0 {
        println!("Select an option: ");
        io::stdin().read_line(&mut user_input_str).expect("Error reading input, Try again");
        user_input = user_input_str.trim().parse().unwrap();
        if user_input == 1 {

        }
        else if user_input == 2 {
            let _user_address = create_new_address(network);
        }
        else if user_input == 3 {
            let mut pubkey = String::new();
            io::stdin().read_line(&mut pubkey).expect("Error reading input, Try again");
            let address_data = get_address_data(pubkey);
            println!("Address: {:?}", address_data)
        }
        else {
            println!("Please input a valid argument.")
        }
    }
    //Will implement later:
    // let serialized = serialize_address(_user_address);

    // let deserialized = deserialize_address(serialized);

}