use crate::{types::chain::epoch::ChainEpoch, CID};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SectorPreCommitInfo {
    pub SealProof: i64,
    pub SectorNumber: u64,
    pub SealedCID: CID,
    pub SealRandEpoch: ChainEpoch,
    pub DealIDs: Option<Vec<DealId>>,
    pub Expiration: ChainEpoch,
    pub UnsealedCid: Option<CID>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SectorPreCommitOnChainInfo {
    pub Info: SectorPreCommitInfo,
    pub PreCommitDeposit: TokenAmount,
    pub PreCommitEpoch: ChainEpoch,
}

// TokenAmount is the amount in parts of token. In filecoin that's 1e-18 per one part of a token.
// In Lotus this is using big.Int which are arbitrary integers.
// Once token arithmetics would be needed we could investigate how to implement this as an
// arbitrary integer/
pub type TokenAmount = String;

// Similarly to TokenAmount, this also needs to use arbitrary integers.
pub type DealId = String;
