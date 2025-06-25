# 🧮 Solana Counter Program

A minimal, educational Solana smart contract written in Rust that demonstrates on-chain integer counter logic. This program lets users increment, decrement, multiply, or divide a counter stored in a Solana account using Borsh serialization. No Anchor framework is used—this is a pure Rust, native Solana smart contract.

---

## 🌐 Live Deployment

- **Program ID:** `9fMw19JQGh63bioSwxujdLMgqDUiRzLuvTnFTaTdjWws`
- **Deployed on:** [devnet](https://solscan.io/account/9fMw19JQGh63bioSwxujdLMgqDUiRzLuvTnFTaTdjWws?cluster=devnet)

---

## 📦 Features

- **Increment/Decrement/Multiply/Divide**: On-chain integer counter with atomic operations.
- **Borsh Serialization**: Efficient, schema-driven serialization for account data.
- **Native Solana Program**: Pure Rust implementation, no Anchor dependency.
- **Error Handling**: Checks for overflow/underflow and division by zero.

---

## 📁 Project Structure

```
caloc_Solana_Native/
├── Cargo.toml            # Rust package manifest for Solana program
└── src/
    └── lib.rs            # Main program logic and Solana entrypoint
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Borsh crate](https://crates.io/crates/borsh)
- Solana toolchain configured for BPF:  
  `rustup target add bpfel-unknown-unknown`

---

### 🛠 Build

To build the Solana smart contract in BPF form:

```bash
cargo build-bpf
```

This outputs the program binary at `target/deploy/sol-program-counter.so`.

---

### 🚀 Deploy

To deploy to Solana devnet:

```bash
solana program deploy target/deploy/sol-program-counter.so
```

Or use the pre-deployed Program ID:  
`9fMw19JQGh63bioSwxujdLMgqDUiRzLuvTnFTaTdjWws` on devnet.

---

## 🧩 Example: Serialization Logic

The program uses Borsh for serialization. Example for a similar struct:

```rust
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}
```

## 🛠️ Instruction Set

The program supports the following instructions:

- `Increment(u32)`: Adds value to the counter.
- `Decrement(u32)`: Subtracts value from the counter.
- `Multiply(u32)`: Multiplies the counter.
- `Divide(u32)`: Divides the counter (with division by zero check).

All instructions are dispatched in the main entrypoint and update the on-chain `Counter` account.

---

## 🗺️ Roadmap

- [x] On-chain counter with increment/decrement/multiply/divide
- [x] Borsh serialization demo
- [ ] CLI or JavaScript client for on-chain interaction (coming soon)
- [ ] Add reset/read instructions for the counter

---

## 👨‍💻 Author

Made by [Raunit Jaiswal](https://github.com/raunit-dev)

---

## 📝 License

This project currently does not specify a license. Please add one before using it in production.

---

## 🤝 Contributing

Pull requests are welcome for improvements, documentation, or additional features!
