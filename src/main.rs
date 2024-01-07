#![allow(non_snake_case)]
use bdk::bitcoincore_rpc::{Auth as rpc_auth, Client, RpcApi};
use std::io::{self, Write};



fn main() {

    let rpc_auth = rpc_auth::CookieFile("/home/orangepi/.bitcoin/.cookie".into());

    let mut input = String::new();
    print!("Please enter wallet label: ");
    io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    let url = format!("http://127.0.0.1:18443/wallet/{}", input);

    let core_rpc = Client::new(&url, rpc_auth).unwrap();

    let core_balance = core_rpc.get_balance(None, None).unwrap();
    let wallet_info = core_rpc.get_wallet_info().unwrap();
    let utxo_list = core_rpc.list_unspent(None, None, None, None, None).unwrap();

    println!("\nBalance: {:?}", core_balance);
    println!("HD Seed ID: {:?}", wallet_info.hd_seed_id);
    println!("Private Keys Enabled: {}", wallet_info.private_keys_enabled);
    println!("Avoid Reuse: {:?}", wallet_info.avoid_reuse.unwrap_or(false));
    println!("Unlocked Until: {:?}\n", wallet_info.unlocked_until);

    for utxo in &utxo_list {
        match &utxo.address {
            Some(address) => {
                let descriptor = utxo.descriptor.as_ref().map_or("No descriptor", |d| d.as_str());
                println!("{:?}, {}\n{}\n", address, utxo.amount, descriptor)
            },
            None => println!("No address found for this UTXO."),
        }
    }
}
