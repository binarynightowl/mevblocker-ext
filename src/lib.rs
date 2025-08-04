//! # MEVBlocker Alloy Extension
//! 
//! This crate provides an extension to the Alloy Ethereum library to support
//! MEVBlocker's custom RPC methods for subscribing to unsigned pending transactions.
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! mevblocker-ext = "0.1.0"
//! alloy = { version = "1.0", features = ["pubsub"] }
//! ```
//! 
//! Then use the `MevblockerApi` trait:
//! 
//! ```rust,no_run
//! use alloy::providers::{Provider, ProviderBuilder, WsConnect};
//! use futures_util::StreamExt;
//! use mevblocker_ext::MevblockerApi;
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let rpc_url = "wss://searchers.mevblocker.io";
//! let ws = WsConnect::new(rpc_url);
//! let provider = ProviderBuilder::new().connect_ws(ws).await?;
//! 
//! let sub = provider.subscribe_mevblocker_pending_transactions().await?;
//! let mut stream = sub.into_stream();
//! 
//! while let Some(tx) = stream.next().await {
//!     println!("Unsigned transaction: {tx:#?}");
//! }
//! # Ok(())
//! # }
//! ```

mod mevblocker;

pub use mevblocker::MevblockerApi;