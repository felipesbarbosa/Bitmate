//Will implement later:
/*
use bitcoin::bip32::{ChildNumber, ExtendedPubKey};
use bitcoin::taproot::{tapscript::TapScript, TaprootScript};
*/
use bitcoin::{network::constants::Network,};

mod address;
use address::{create_new_address};

fn main() {
    //Chosing the desired bitcoin network 
    let network = Network::Testnet;
    
    let _user_address = create_new_address(network);

    //Will implement later:
    // let serialized = serialize_address(_user_address);

    // let deserialized = deserialize_address(serialized);

}