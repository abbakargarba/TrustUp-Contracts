# TrustUp Contracts

TrustUp is a decentralized "Buy Now, Pay Later" (BNPL) platform built on the Stellar network using Soroban smart contracts. Users pay a 20% guarantee deposit and receive credit from a shared liquidity pool, with their on-chain reputation adjusting based on repayment behavior.

## Project Structure

```
trustup-contracts/
├── Cargo.toml                 # Workspace configuration
├── contracts/
│   ├── escrow-contract/       # Manages user guarantees and merchant payments
│   ├── creditline-contract/   # Handles loan creation and repayment
│   ├── reputation-contract/   # Tracks and updates user reputation scores
│   └── pool-contract/         # Manages liquidity provider deposits and rewards
├── README.md
└── .gitignore
```

## Contracts Overview

### Escrow Contract
Holds user guarantees (20% down payment) and releases funds to merchants when transaction conditions are met. Manages the complete escrow lifecycle for BNPL transactions.

**Key responsibilities:**
- Deposit and hold user guarantee payments
- Release funds to merchants upon fulfillment
- Handle refunds when applicable
- Track escrow status

### CreditLine Contract
Handles loan creation and repayment processing for BNPL transactions. Manages credit issuance, payment schedules, and repayment tracking.

**Key responsibilities:**
- Create and issue credit lines
- Process loan repayments
- Manage payment schedules
- Calculate interest and fees
- Track loan status

### Reputation Contract
Tracks and updates user reputation scores based on repayment behavior. User reputation adjusts dynamically based on their on-chain payment history.

**Key responsibilities:**
- Track user reputation scores
- Update reputation based on repayment performance
- Provide reputation queries
- Maintain reputation history
- Determine credit limits based on reputation

### Pool Contract
Manages liquidity provider deposits and rewards. Handles the shared liquidity pool that funds BNPL credit lines.

**Key responsibilities:**
- Accept liquidity provider deposits
- Process liquidity withdrawals
- Distribute rewards to providers
- Manage pool balance and allocation
- Set interest rates

## Prerequisites

- Rust (latest stable version)
- Soroban SDK (included via Cargo dependencies)
- `wasm32-unknown-unknown` target for Rust

Install the required target:
```bash
rustup target add wasm32-unknown-unknown
```

## Building

Build all contracts:
```bash
cargo build --target wasm32-unknown-unknown --release
```

Build a specific contract:
```bash
cargo build --target wasm32-unknown-unknown --release -p escrow-contract
cargo build --target wasm32-unknown-unknown --release -p creditline-contract
cargo build --target wasm32-unknown-unknown --release -p reputation-contract
cargo build --target wasm32-unknown-unknown --release -p pool-contract
```

## Testing

Run all tests:
```bash
cargo test
```

Run tests for a specific contract:
```bash
cargo test -p escrow-contract
cargo test -p creditline-contract
cargo test -p reputation-contract
cargo test -p pool-contract
```

## Adding New Contracts

To add a new contract to the workspace:

1. Create a new folder in `contracts/` with your contract name
2. Add a `Cargo.toml` with Soroban SDK dependency (version 22.0.0+)
3. Add a `src/lib.rs` with contract skeleton
4. Add the contract member to the root `Cargo.toml` workspace members list

Example `Cargo.toml` for a new contract:
```toml
[package]
name = "my-new-contract"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
testutils = ["soroban-sdk/testutils"]
default = []

[dependencies]
soroban-sdk = "22.0.0"

[dev-dependencies]
soroban-sdk = { version = "22.0.0", features = ["testutils"] }
```

## Development Status

This is the initial project structure and boilerplate. Contract logic implementation is pending.

**Current version:** trustup-v1.0

## License

[Add your license here]
# TrustUp-Contracts
