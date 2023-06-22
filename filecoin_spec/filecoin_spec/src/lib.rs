use serde::{Deserialize, Serialize};
use serde_with::{
    base64::{Base64, Standard},
    formats::Padded,
    serde_as,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub struct UnpaddedBytesAmount(pub u64);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PieceInfo {
    pub commitment: Commitment,
    pub size: UnpaddedBytesAmount,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum RegisteredSealProof {
    StackedDrg2KiBV1,
    StackedDrg8MiBV1,
    StackedDrg512MiBV1,
    StackedDrg32GiBV1,
    StackedDrg64GiBV1,

    StackedDrg2KiBV1_1,
    StackedDrg8MiBV1_1,
    StackedDrg512MiBV1_1,
    StackedDrg32GiBV1_1,
    StackedDrg64GiBV1_1,
}

#[serde_as]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct Commitment(#[serde_as(as = "Base64<Standard, Padded>")] pub [u8; 32]);

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Ticket(#[serde_as(as = "Base64<Standard, Padded>")] pub [u8; 32]);

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChainEpoch(pub i64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ReplicaId(pub Vec<u8>);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct ProverId(pub [u8; 32]);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SectorId(pub u64);

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct VanillaProof(pub Vec<u8>);

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct StorageProviderId(pub u64);

impl From<StorageProviderId> for ProverId {
    fn from(sp_id: StorageProviderId) -> Self {
        let mut prover_id = ProverId::default();
        let mut buf = unsigned_varint::encode::u64_buffer();
        let buf = unsigned_varint::encode::u64(sp_id.0, &mut buf);
        prover_id.0[..buf.len()].copy_from_slice(buf);
        prover_id
    }
}

impl From<ProverId> for StorageProviderId {
    fn from(prover_id: ProverId) -> Self {
        let sp_id = unsigned_varint::decode::u64(&prover_id.0).unwrap().0;

        StorageProviderId(sp_id)
    }
}

impl From<&ProverId> for StorageProviderId {
    fn from(prover_id: &ProverId) -> Self {
        let sp_id = unsigned_varint::decode::u64(&prover_id.0).unwrap().0;

        StorageProviderId(sp_id)
    }
}

pub const SECTOR_SIZE_2_KIB: u64 = 1 << 11;
pub const SECTOR_SIZE_8_MIB: u64 = 1 << 23;
pub const SECTOR_SIZE_512_MIB: u64 = 1 << 29;
pub const SECTOR_SIZE_32_GIB: u64 = 1 << 35;
pub const SECTOR_SIZE_64_GIB: u64 = 1 << 36;

pub const fn sector_size(proof: &RegisteredSealProof) -> u64 {
    match proof {
        RegisteredSealProof::StackedDrg2KiBV1 => SECTOR_SIZE_2_KIB,
        RegisteredSealProof::StackedDrg8MiBV1 => SECTOR_SIZE_8_MIB,
        RegisteredSealProof::StackedDrg512MiBV1 => SECTOR_SIZE_512_MIB,
        RegisteredSealProof::StackedDrg32GiBV1 => SECTOR_SIZE_32_GIB,
        RegisteredSealProof::StackedDrg64GiBV1 => SECTOR_SIZE_64_GIB,
        RegisteredSealProof::StackedDrg2KiBV1_1 => SECTOR_SIZE_2_KIB,
        RegisteredSealProof::StackedDrg8MiBV1_1 => SECTOR_SIZE_8_MIB,
        RegisteredSealProof::StackedDrg512MiBV1_1 => SECTOR_SIZE_512_MIB,
        RegisteredSealProof::StackedDrg32GiBV1_1 => SECTOR_SIZE_32_GIB,
        RegisteredSealProof::StackedDrg64GiBV1_1 => SECTOR_SIZE_64_GIB,
    }
}
