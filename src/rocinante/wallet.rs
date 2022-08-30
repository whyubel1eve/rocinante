use std::fs::OpenOptions;
use std::io::BufWriter;
use std::str::FromStr;
use colorful::{Color};
use colorful::Colorful;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use secp256k1::rand::rngs::OsRng;
use web3::signing::keccak256;
use web3::types::{Address};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Wallet {
    address: String,
    private_key: String,
    public_key: String,
}

enum Network {
    Main,
    Goerli,
}
impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "main" => Ok(Network::Main),
            "goerli" => Ok(Network::Goerli),
            _ => Err(format!("'{}' is not supported", s)),
        }
    }
}

pub async fn new() -> web3::Result<()> {
    let (private_key, public_key) = generate_keypair();
    let address = generate_address(&public_key);

    let wallet = Wallet {
        private_key: format!("{}", private_key.display_secret()),
        public_key: public_key.to_string(),
        address: format!("{:?}", address),
    };
    let buf = BufWriter::new(OpenOptions::new().write(true).create(true).open("account.json")?);
    serde_json::to_writer_pretty(buf, &wallet).unwrap();

    println!("{}", format!("address: {:?}", address).color(Color::LightCyan));

    Ok(())
}

pub async fn balance(addr: &String, network: &String) -> web3::Result<()> {
    let api;
    let network = Network::from_str(network).unwrap();

    match network {
        Network::Main=> { api = "https://mainnet.infura.io/v3/4828ef93cc7346c9af614be8e52c440b" }
        Network::Goerli => { api = "https://goerli.infura.io/v3/4828ef93cc7346c9af614be8e52c440b" }
    }
    let transport = web3::transports::Http::new(api)?;
    let web3 = web3::Web3::new(transport);

    let balance = web3.eth().balance(Address::from_str(addr).unwrap(), None).await?;

    println!("{}", format!("Balance: {} wei", balance).color(Color::LightCyan));

    Ok(())
}

fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    secp.generate_keypair(&mut OsRng)
}

fn generate_address(pub_key: &PublicKey) -> Address {
    let pub_key = pub_key.serialize_uncompressed();
    debug_assert_eq!(pub_key[0], 0x04);
    let hash = keccak256(&pub_key[1..]);
    Address::from_slice(&hash[12..])
}