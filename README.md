# MEVBlocker Alloy Extension

A Rust library that extends [Alloy](https://github.com/alloy-rs/alloy) with MEVBlocker's custom RPC methods for subscribing to unsigned transactions.

## Overview

This library provides a trait extension for Alloy providers to support MEVBlocker's `mevblocker_partialPendingTransactions` subscription, which streams unsigned pending transactions missing signature fields (v, r, s).

## Features

- 🔌 Seamless integration with Alloy providers
- 📡 Subscribe to MEVBlocker's partial pending transactions via WebSocket
- 🎯 Deserializes transaction data into Alloy's `TransactionRequest` type
- ⚡ Compatible with Alloy's async streaming interface

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mevblocker-ext = "0.1.0"
alloy = { version = "1.0", features = ["pubsub"] }
futures-util = "0.3"
tokio = { version = "1.0", features = ["rt", "macros"] }
eyre = "0.6"
```

## Usage

```rust
use alloy::providers::{ProviderBuilder, WsConnect};
use futures_util::StreamExt;
use mevblocker_ext::MevblockerApi;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "wss://searchers.mevblocker.io";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let sub = provider.subscribe_mevblocker_pending_transactions().await?;
    let mut stream = sub.into_stream();

    while let Some(tx) = stream.next().await {
        println!("Unsigned transaction: {tx:#?}");
    }

    Ok(())
}
```

## Running the Example

```bash
cargo run --example subscribe
```

## License

Licensed under MIT license.
