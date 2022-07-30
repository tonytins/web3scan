pub mod models;

use crate::models::*;
use reqwest::Error;
use std::env;
use std::path::Path;

fn get_api_key() -> String {
    if Path::new(".env").exists() {
        dotenv::dotenv().expect("Error parsing .env file!");
    }

    let api_key = env::var("APIKEY").expect("Failed to locate API key!");

    return format!("&apikey={}", api_key);
}

pub struct Web3 {
    /// Etherscan-based API address without HTTP included.
    provider: String,
}

impl Web3 {
    pub fn new<S: Into<String>>(provider: S) -> Self {
        Web3 {
            provider: format!("https://{}/api?module=", provider.into()),
        }
    }

    /// Retrieves the client's Ethereum balance as Wei.
    ///
    /// ## Example
    /// ```rust
    /// use web3scan::Web3;
    ///
    /// fn main() {
    ///
    /// let network = Web3::new("api.etherscan.io");
    /// let balance = network.get_balance("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B")
    ///                 .expect("parser error");
    ///
    /// println!("{}", balance);
    ///
    /// }
    /// ```
    pub async fn get_balance<S: Into<String>>(&self, address: S) -> Result<String, Error>
    where
        S: Into<String>,
    {
        let request = format!(
            "{}account&action=balance&address={}&tag=latest{}",
            self.provider,
            address.into(),
            get_api_key().as_str()
        );
        let response = reqwest::get(&request)
            .await?
            .json::<Etherscan<String>>()
            .await?;
        let balance: Etherscan<String> = response;

        Ok(balance.result)
    }

    /// Retrieves the client's Ethereum balance as Wei.
    ///
    /// ## Example
    /// ```rust
    /// use web3scan::Web3;
    ///
    /// fn main() {
    ///
    /// let network = Web3::new("api.etherscan.io");
    /// let total_supply = network.get_total_supply().expect("parser error");
    ///
    /// println!("{}", total_supply);
    ///
    /// }
    /// ```
    pub async fn get_total_supply(&self) -> Result<String, Error> {
        let request = format!(
            "{}stats&action=ethsupply{}",
            self.provider,
            get_api_key().as_str()
        );
        let response = reqwest::get(&request)
            .await?
            .json::<Etherscan<String>>()
            .await?;
        let taken_supply: Etherscan<String> = response;

        Ok(taken_supply.result)
    }

    /// Retrieves the client's Ethereum balance as Wei.
    ///
    /// ## Example
    /// ```rust
    /// use web3scan::Web3;
    ///
    /// fn main() {
    ///
    /// let network = Web3::new("api.etherscan.io");
    /// let total_supply = network.get_total_supply().expect("parser error");
    ///
    /// println!("{}", total_supply);
    ///
    /// }
    /// ```
    pub async fn get_last_price(&self) -> Result<EvmPrice, Error> {
        let request = format!(
            "{}stats&action=ethprice{}",
            self.provider,
            get_api_key().as_str()
        );
        let response = reqwest::get(&request)
            .await?
            .json::<Etherscan<EvmPrice>>()
            .await?;
        let last_price: Etherscan<EvmPrice> = response;

        Ok(last_price.result)
    }

    /// Retrieves the client's Ethereum balance as Wei.
    ///
    /// ## Example
    /// ```rust
    /// use web3scan::Web3;
    ///
    /// fn main() {
    ///
    /// let network = Web3::new("api.etherscan.io");
    /// let last_price = network.get_last_price().expect("parser error");
    ///
    /// println!("{}", last_price);
    ///
    /// }
    /// ```
    pub async fn get_transactions<S: Into<String>>(
        &self,
        address: S,
        start_block: i64,
        end_block: i64,
    ) -> Result<Vec<EvmTransaction>, Error>
    where
        S: Into<String>,
    {
        let request = format!(
            "{}account&action=txlist&address={}&startblock={}&endblock={}&sort=asc{}",
            self.provider,
            address.into(),
            start_block,
            end_block,
            get_api_key().as_str()
        );

        let response = reqwest::get(&request)
            .await?
            .json::<Etherscan<Vec<EvmTransaction>>>()
            .await?;

        let balance: Etherscan<Vec<EvmTransaction>> = response;

        Ok(balance.result)
    }
}
