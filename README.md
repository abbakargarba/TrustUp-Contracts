
# TrustUp Contracts

Decentralized "Buy Now, Pay Later" (BNPL) platform on Stellar using Soroban smart contracts. Users pay 20% guarantee deposit and receive credit from a shared liquidity pool, with on-chain reputation adjusting based on repayment behavior.

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

## Build

Check contracts (verify without building):
```bash
cargo check
```

Build all contracts:
```bash
cargo build --release
```

Build a specific contract:
```bash
cargo build --release -p escrow-contract
cargo build --release -p creditline-contract
cargo build --release -p reputation-contract
cargo build --release -p pool-contract
```

Build for development:
```bash
cargo build
```

## Test

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

## Development Status

This is the initial project structure and boilerplate. Contract logic implementation is pending.

**Current version:** trustup-v1.0
