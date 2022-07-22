# Web3Scan

[![GitHub license](https://img.shields.io/github/license/tonytins/web3scan)](https://github.com/tonytins/web3scan/blob/main/LICENSE) ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/tonytins/web3scan/Rust/main) ![GitHub commit activity](https://img.shields.io/github/commit-activity/w/tonytins/web3scan)  [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct.md)

A descendant from the [CoinMarket](https://crates.io/crates/coinmarket) library, Web3Scan is a used for gathering information from EVM-based networks using Etherscan's API.

## Installation

```toml
[dependencies]
coinmarket = "0.1"
```

## Example

```env
APIKEY=[key]
```

```rust
use web3scan::Web3;

pub fn main() {
    let network = Web3::new("api.etherscan.io");
    let balance = network
    .get_balance("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B")
    .expect("Parsing error");

    println!("{}", balance);
}
```

## Requirements

- Rust 2021 Edition or later
- IDEs or Editors
  - [Visual Studio Code](https://code.visualstudio.com/)
  - [InteliJ IDEA](https://www.jetbrains.com/idea/) or [CLion](https://www.jetbrains.com/clion/)

## License

This project is dual-licensed under the [BSD-3-Clause](COPYING) or the [UNLICENSE](UNLICENSE).
