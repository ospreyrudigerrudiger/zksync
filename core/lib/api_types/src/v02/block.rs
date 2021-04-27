use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zksync_crypto::{serialization::FrSerde, Fr};
use zksync_types::{BlockNumber, H256};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BlockStatus {
    Queued,
    Committed,
    Finalized,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BlockInfo {
    pub block_number: BlockNumber,
    #[serde(with = "FrSerde")]
    pub new_state_root: Fr,
    pub block_size: u64,
    pub commit_tx_hash: Option<H256>,
    pub verify_tx_hash: Option<H256>,
    pub committed_at: Option<DateTime<Utc>>,
    pub finalized_at: Option<DateTime<Utc>>,
    pub status: BlockStatus,
}
