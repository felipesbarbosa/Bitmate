//Will implement later:
/*
use bitcoin::bip32::{ChildNumber, ExtendedPubKey};
use bitcoin::taproot::{tapscript::TapScript, TaprootScript};
*/
use bitcoin::{network::constants::Network};
use std::io;

mod address;
mod blockchain;
use address::{create_new_address};
use blockchain::{get_address_data};

// pub struct user_preferences {
//     fiat_currency: String,
//     lang: String,
//     bitscale: String,
//     default_pref: bool
// }

// fn default_config(user_pref:user_preferences){
//     user_pref::fiat_currency = get_fiat_location();
//     user_pref::lang = get_lang_location();
//     user_pref::bitscale = "Sats";
//     user_pref::default_pref = true;
// }
fn main() {
    //Chosing the desired bitcoin network 
    let network = Network::Testnet;
    let mut user_input_str = String::new();
    let mut user_input:i8 = 5;
    println!("Welcome to freedom!\nThis is an app in development for using bitcoin");
    println!("");
    println!("Choose a option bellow:");
    println!("1 - Select network (Default: Testnet)");
    println!("2 - Create new address");
    println!("3 - Read bitcoin address info");
    // println!("4 - Configurations");
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
            let mut search_addr = String::new();
            println!("Insert the address: ");
            io::stdin().read_line(&mut search_addr).expect("Error reading input, Try again");
            let address_data = get_address_data(search_addr);
            println!("Address: {:?}", address_data);
        }
        // else if user_input == 4 {
        //     io::stdin().read_line(&mut user_input_str).expect("Error reading input, Try again");
        //     user_input = user_input_str.trim().parse().unwrap();
        //     println!("1 - Default configurations");
        //     println!("2 - Change language");
        //     println!("3 - Change bitcoin visualization (Sats/Bitcoin)");
        //     println!("4 - Configurations");
        //     println!("0 - Back to main menu");

        // }
        else {
            println!("Please input a valid argument.");
        }
    }
    //Will implement later:
    // let serialized = serialize_address(_user_address);

    // let deserialized = deserialize_address(serialized);

}