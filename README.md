# Web3Scan

Web3Scan was originally intended as a direct descendant of [CoinMarket](https://github.com/tonytins/coinmarket) library, focused on interaction with Web3 APIs from Etherscan, and other similar platforms. However, due to [ongoing events](https://web3isgoinggreat.com) in the Cryptocurrency and DeFi sector, such as hacks, rug pulls, exchanges collapsing and other events following the aftermath of the Terra blockchain's [crash](https://www.cnbc.com/2022/05/13/cryptocurrency-luna-crashes-to-0-as-ust-falls-from-peg-bitcoin-rises.htmls), the project is now deprecated and is no longer maintained.

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
