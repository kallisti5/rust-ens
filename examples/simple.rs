extern crate ens;
extern crate web3;
extern crate tokio;

use web3::types::Address;
use ens::ENS;

#[tokio::main]
async fn main() {
    let transport = web3::transports::http::Http::new(
        "http://localhost:8545",
    ).unwrap();

    let ens_name = "terarocket.eth";

    let ens = ENS::new(web3::Web3::new(transport));
    let addr = match ens.address(ens_name) {
        Ok(a) => a,
        Err(_) => Address::zero(),
    };
    let owner_addr = {
        ens.owner(ens_name).expect("ens.owner() error")
    };
    let reverse_addr = match ens.name(addr) {
        Ok(name) => name,
        Err(_) => "unknown".to_string(),
    };

    println!("name      : {}", ens_name);
    println!("owner_addr: {:?}", owner_addr);
    println!("addr      : {:?}", addr);
    println!("rev_addr  : {}", reverse_addr);
}
