# 🧮 Solana Counter Program

A simple, minimal Solana smart contract written in Rust that demonstrates how to increment or decrement a counter value stored in an on-chain account. The program uses [Borsh](https://borsh.io/) for serialization, and is designed as a native Solana program without the use of frameworks like Anchor.

---

## 📦 Features

- **Increment / Decrement Counter**: Allows on-chain storage and modification of a simple integer counter via transactions.
- **Borsh Serialization**: Utilizes Borsh for efficient, schema-driven serialization of account data.
- **Native Solana Program**: Pure Rust implementation for educational or starter purposes, with no Anchor dependency.

---

## 📁 Project Structure

```
sol-program-counter/
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
- Solana toolchain configured for BPF target:  
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

To deploy to your Solana cluster:

```bash
solana program deploy target/deploy/sol-program-counter.so
```

---

## 🧩 Example: Serialization Logic

The program demonstrates Borsh serialization for a simple `User` struct:

```rust
use borsh::{BorshDeserialize, BorshSerialize, to_vec};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct User {
    name: String,
    age: u32
}

fn main() {
    let u = User {
        name: String::from("Raunit"),
        age: 21,
    };

    let bytes = to_vec(&u).unwrap();
    let u2 = User::try_from_slice(&bytes).unwrap();
    println!("{:?}", u);
    println!("{:?}", u2);
}
```

---

## 🗺️ Roadmap

- [x] On-chain counter with increment/decrement
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
