use web3scan::Web3;

pub fn main() {
    let address = "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B";
    let err_msg = "Parsing error";
    let network = Web3::new("api.etherscan.io");
    let balance = network.get_balance(address).expect(err_msg);
    let total_supply = network.get_total_supply().expect(err_msg);
    let last_price = network.get_last_price().expect(err_msg);

    println!("Balance: {:#?}", balance);
    println!("Total supply: {:#?}", total_supply);
    println!("{:#?}", last_price);
}
