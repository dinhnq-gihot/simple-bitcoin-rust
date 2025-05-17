# Bitcoin Rust (Toy Blockchain)

This project is a toy implementation of a Bitcoin-like blockchain system, written in Rust. It is organized as a Cargo workspace with several components:

## Workspace Structure

- **lib (btclib):** Core blockchain library. Implements cryptography, block/transaction types, networking, consensus rules, and utility functions.
- **node:** Runs a blockchain node. Handles networking, blockchain storage, mempool management, and peer discovery.
- **miner:** Connects to a node, fetches block templates, mines blocks, and submits mined blocks back to the node.
- **wallet:** (Currently a placeholder) Intended for wallet functionality in the future.

## Getting Started

### Prerequisites

- Rust (latest stable, see `rust-toolchain.toml`)
- Cargo

### Build the Workspace

```sh
cargo build --workspace
```

### Running a Node

```sh
cd node
cargo run -- [OPTIONS]
```

**Options:**

- `--port <PORT>`: Port to listen on (default: 9000)
- `--blockchain-file <PATH>`: Path to blockchain file (default: ./data/blockchain.cbor)
- `[NODES]`: Addresses of initial peer nodes

Example:

```sh
cargo run -- --port 9000 --blockchain-file ./data/blockchain.cbor
```

### Running a Miner

```sh
cd miner
cargo run -- [OPTIONS]
```

**Options:**

- `--address <ADDR>`: Node address to connect to (default: 127.0.0.1:9000)
- `--public-key-file <PATH>`: Path to miner's public key PEM file (default: ./accounts/alice.pub.pem)

Example:

```sh
cargo run -- --address 127.0.0.1:9000 --public-key-file ./accounts/alice.pub.pem
```

### Wallet

The wallet crate is currently a placeholder and does not provide functionality yet.

## Features

- Basic blockchain and block validation
- Peer-to-peer networking
- Mining with adjustable difficulty
- Block and transaction serialization
- Cryptographic signatures (ECDSA)

## Development

- Format code: `cargo fmt`
- Run tests: `cargo test --workspace`

## License

MIT
