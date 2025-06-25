# 🧮 Solana Counter Program

A simple Solana smart contract written in Rust that increments or decrements a counter stored in an account. Uses Borsh for serialization.

## 📦 Features

- Increment / Decrement counter stored on-chain
- Borsh-based serialization
- Minimal, native Solana program (no Anchor)

## 📁 Structure

sol-program-counter/
├── Cargo.toml
└── src/
└── lib.rs


## 🛠 Build

```bash
cargo build-bpf


🚀 Deploy

solana program deploy target/deploy/sol-program-counter.so

🧪 Coming Soon
CLI or JS client to test it

Reset / Read instructions

👨‍💻 Author
Made by Raunit Jaiswal
