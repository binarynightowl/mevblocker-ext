use alloy::providers::{ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;
use mevblocker_ext::MevblockerApi;

#[tokio::main]
async fn main() -> Result<()> {

    println!("\n=== MEVBlocker Pending Transactions ===");
    let mevblocker_rpc_url = "wss://searchers.mevblocker.io";
    let mevblocker_ws = WsConnect::new(mevblocker_rpc_url);
    let mevblocker_provider = ProviderBuilder::new().connect_ws(mevblocker_ws).await?;

    let mevblocker_sub = mevblocker_provider.subscribe_mevblocker_pending_transactions().await?;
    let mut mevblocker_stream = mevblocker_sub.into_stream().take(10);

    println!("Awaiting MEVBlocker pending transactions...");

    let mevblocker_handle = tokio::spawn(async move {
        while let Some(tx) = mevblocker_stream.next().await {
            println!("MEVBlocker pending transaction: \n{tx:#?}\n---");
        }
    });

    mevblocker_handle.await?;

    Ok(())
}
