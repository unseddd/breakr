# breakr
Ethereum (Classic) smart contract fuzzer

## Requirements

- Rust nightly 1.33+
- cargo-fuzz
- Etherscan API key (for downloading smart contract ABI)

## Installation

Clone the repo and build:

```
git clone https://github.com/unseddd/breakr
cd breakr
cargo build
```

## Usage

Currently, `breakr` uses Etherscan to download smart contract ABI binary, which requires an account and API key.

Once you have your API key, you can set it in `Settings.toml` by copying the `examples/Settings.toml` to the crate root,
and replacing the dummy value with your key.

Now you can download and fuzz a contract's ABI from a block explorer:

```
cargo run -- --contract 0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359
```

To run the fuzzer on a local smart contract ABI, copy the ABI to the file `fuzz/contracts/fuzz.bin`, and run:

```
cargo fuzz run evm
```
