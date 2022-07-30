use reqwest::Error;
use web3scan::Web3;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let address = "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B";
    let network = Web3::new("api.etherscan.io");
    let balance = network.get_balance(address).await?;
    let total_supply = network.get_total_supply().await?;
    let last_price = network.get_last_price().await?;

    println!("Balance: {:#?}", balance);
    println!("Total supply: {:#?}", total_supply);
    println!("{:#?}", last_price);

    Ok(())
}
