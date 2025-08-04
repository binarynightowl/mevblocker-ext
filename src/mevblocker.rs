//! This module extends the Ethereum JSON-RPC provider with MEVBlocker's custom RPC methods.
use alloy::network::Network;
#[cfg(feature = "pubsub")]
use alloy::providers::GetSubscription;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;

/// Mevblocker rpc interface that gives access to several non-standard RPC methods offered by mevblocker
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
pub trait MevblockerApi<N>: Send + Sync {
    /// Subscribe to MEVBlocker partial pending transactions.
    #[cfg(feature = "pubsub")]
    fn subscribe_mevblocker_pending_transactions(
        &self,
    ) -> GetSubscription<(String,), TransactionRequest>;
}

#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
impl<N, P> MevblockerApi<N> for P
where
    N: Network,
    P: Provider<N>,
{
    #[cfg(feature = "pubsub")]
    fn subscribe_mevblocker_pending_transactions(
        &self,
    ) -> GetSubscription<(String,), TransactionRequest> {
        let mut rpc_call = self.client().request(
            "eth_subscribe",
            ("mevblocker_partialPendingTransactions".to_string(),),
        );
        rpc_call.set_is_subscription();
        GetSubscription::new(self.weak_client(), rpc_call)
    }
}
