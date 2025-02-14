use api_types::api::transactions::{ApiApplicationMetadata, ApiTxStatus};
use chrono::{LocalResult, TimeZone, Utc};
use iota_sdk::{
    types::block::{helper::network_name_to_id, output::Output, payload::transaction::TransactionEssence},
    wallet::account::types::Transaction,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Transaction list
#[derive(Debug, Serialize)]
pub struct TxList {
    /// List of transaction info
    pub txs: Vec<TxInfo>,
}

/// Transaction info
#[derive(Debug, Serialize, Clone)]
pub struct TxInfo {
    /// Tx creation date, if available
    pub date: Option<String>,
    /// sender of the transaction
    pub sender: String,
    /// receiver of the transaction
    pub receiver: String,
    /// cryptpay reference id for the transaction
    pub reference_id: String,
    /// Application specific metadata attached to the tx
    pub application_metadata: Option<ApiApplicationMetadata>,
    /// Amount of transfer
    pub amount: f64,
    /// Currency of transfer
    pub currency: String,
    /// Status of the transfer
    pub status: ApiTxStatus,
    /// The transaction hash on the network
    pub transaction_hash: Option<String>,
    /// Exchange rate
    pub course: f64,
}
/// wallet transaction info
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WalletTxInfo {
    /// Tx creation date, if available
    pub date: String,
    /// Contains block id
    pub block_id: Option<String>,
    /// transaction id for particular transaction
    pub transaction_id: String,
    /// Describes type of transaction
    pub incoming: bool,
    /// Amount of transfer
    pub amount: f64,
    /// either ETH, SMR or IOTA [convert network_id to string based on the value]
    pub network: String,
    /// Status of the transfer
    pub status: String,
    /// Url of network SMR/IOTA/ETH
    pub explorer_url: Option<String>, // ok
                                      // change based on the network either shimmer or iota
                                      // base explorer url for IOTA = https://explorer.iota.org/mainnet/block/[block_id]
                                      // base explorer url for EVM = [node url]
}

/// List of wallet transactions
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct WalletTxInfoList {
    /// Transactions that happens
    pub transactions: Vec<WalletTxInfo>,
}

impl From<Transaction> for WalletTxInfo {
    fn from(transaction: Transaction) -> Self {
        let essence = transaction.payload.essence();

        let TransactionEssence::Regular(regular) = essence;
        let mut sum: u64 = 0u64;
        for output in regular.outputs().iter() {
            if let Output::Basic(output) = output {
                sum += output.amount();
            }
        }
        // convert from GLOW to IOTA/SMR
        let sum = sum as f64 / 1_000_000.0;

        let (network, explorer_url) = match transaction.network_id {
            id if id == network_name_to_id("iota") => {
                let explorer_url = transaction
                    .block_id
                    .map(|id| format!("https://explorer.iota.org/mainnet/block/{id}"));
                ("IOTA", explorer_url)
            }
            id => {
                log::error!("unknown network id: {id}");
                ("", None)
            }
        };

        let date = match Utc.timestamp_millis_opt(transaction.timestamp as i64) {
            LocalResult::Single(timestamp) => timestamp.to_rfc3339(),
            _ => {
                log::error!("could not convert timestamp to date");
                "".to_string()
            }
        };

        WalletTxInfo {
            block_id: transaction.block_id.map(|id| id.to_string()),
            transaction_id: transaction.transaction_id.to_string(),
            incoming: transaction.incoming,
            amount: sum,
            network: network.to_string(),
            status: format!("{:?}", transaction.inclusion_state),
            explorer_url,
            date,
        }
    }
}

/// Purchase details
#[derive(Clone)]
pub struct PurchaseDetails {
    /// The sender address where the fees goes to.
    pub system_address: String,
    /// The amount to be paid.
    pub amount: Decimal,
    /// The status of transaction
    pub status: ApiTxStatus,
}

/// Gas estimation (EIP-1559)
#[derive(Debug, PartialEq)]
pub struct GasCostEstimation {
    /// The maximum fee the sender is willing to pay per unit of gas.
    pub max_fee_per_gas: u128,
    /// The maximum tip the sender is willing to pay to miners (in EIP-1559).
    pub max_priority_fee_per_gas: u128,
    /// The maximum amount of gas that the transaction can consume.
    pub gas_limit: u64,
}
