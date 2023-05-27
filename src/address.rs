use bitcoin::secp256k1::{rand, Secp256k1, SecretKey, KeyPair};
use bitcoin::network::constants::Network;
use bitcoin::key::PrivateKey;
use bitcoin::{Address, PublicKey};
//use std::fs::File;
//use std::io::prelude::*;
//use serde::{Serialize, Deserialize};
//use serde_with::{SerializeWith, DeserializeWith};

//#[derive(Serialize, Deserialize, Debug)]

pub struct Addressinfo {
    public_key: PublicKey,
    private_key: PrivateKey,
    address: Address
}

pub fn create_new_address(network:Network) -> Addressinfo {

    //Creating new ECDSA variable and secret key
    let secp = Secp256k1::new();
    let key_pair = KeyPair::new(&secp, &mut rand::thread_rng());
    let secret_key = SecretKey::from_keypair(&key_pair);
    //Creating new private key
    let pvkey = PrivateKey::new(secret_key, network);
    //Creating a public key from the private key
    let pubkey = PrivateKey::public_key(&pvkey, &secp);
    let address = Address::p2pkh(&pubkey, network);

    let new_address = Addressinfo {
        private_key: pvkey,
        public_key: pubkey,
        address: address
    };

    println!("Private key: {}", new_address.public_key);
    println!("Public key: {}", new_address.private_key);
    println!("Address: {}", new_address.address);

     return new_address;

}

//Will implement later:

// pub fn serialize_address(address:Addressinfo) {

//     // Convert the Point to a JSON string.
//     let serialized = serde_json::to_string(&address).unwrap();

//     println!("serialized = {}", serialized);

//     return serialized;
// }

// pub fn deserialize_address(serialized:String) {

//     // Convert the JSON string back to a Point.
//     let deserialized: Addressinfo = serde_json::from_str(&serialized).unwrap();

//     // Prints deserialized = Point { x: 1, y: 2 }
//     println!("deserialized = {:?}", deserialized);

//     return deserialized;
// }