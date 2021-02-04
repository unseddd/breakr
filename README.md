# breakr
Ethereum (Classic) smart contract fuzzer

## Requirements

- Rust 1.33+
- honggfuzz
- Etherscan API key (for downloading smart contract ABI)

## Installation

Clone the repo and build:

```
git clone https://github.com/unseddd/breakr
cd breakr
cargo build
```

Install honggfuzz:

```
cargo install honggfuzz
```

## Usage

Currently, `breakr` uses Etherscan to download smart contract ABI binary, which requires an account and API key.

Once you have your API key, you can set it in `settings.toml` by copying the `examples/settings.toml` to the crate root,
and replacing the dummy value with your key.

Now you can download and fuzz a contract's ABI from a block explorer:

```
cargo run -- --contract 0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359
```

To run the fuzzer on a local smart contract ABI, copy the ABI to the file `fuzz/contracts/fuzz.bin`, and run:

```
cd /path/to/breakr/fuzz
cargo hfuzz run evm
```

`honggfuzz` also supports using dictionaries for formatted input. For example, you can use the sample dictionary:

```
cd /path/to/breakr/fuzz
HFUZZ_RUN_ARGS="--dict dict/input" cargo hfuzz run evm 
```

Dictionary entries are separated by newline characters, and are used by the fuzzer to generate mutated input.

For smart contracts, this will almost always mean providing function signatures with known-good inputs as arguments (in ABI format).

For more command-line options see `honggfuzz`'s usage documentation: [https://github.com/google/honggfuzz/blob/master/docs/USAGE.md#cmdline---help](https://github.com/google/honggfuzz/blob/master/docs/USAGE.md#cmdline---help)

## Credits

`breakr` is basically just glue-code around more substantive projects.

Namely:

- [SputnikVM](https://github.com/rust-blockchain/evm)
- [web3](https://github.com/tomusdrw/rust-web3)
- [honggfuzz](https://github.com/rust-fuzz/honggfuzz-rs)
