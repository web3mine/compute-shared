use super::epoch::ChainEpoch;
use crate::types::chain::address::Address;
use serde::{Deserialize, Serialize};

pub type PeerId = String;
pub type RegisteredPoStProof = i64;
pub type SectorSize = u64;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MinerInfo {
    pub Owner: Address,
    pub Worker: Address,
    pub NewWorker: Address,
    pub ControlAddresses: Option<Vec<Address>>,
    pub WorkerChangeEpoch: ChainEpoch,
    pub PeerId: PeerId,
    pub Multiaddrs: Option<Vec<String>>,
    pub WindowPoStProofType: RegisteredPoStProof,
    pub SectorSize: SectorSize,
    pub WindowPoStPartitionSectors: u64,
    pub ConsensusFaultElapsed: ChainEpoch,
}
