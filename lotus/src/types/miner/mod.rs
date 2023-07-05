use std::{collections::HashMap, str::FromStr};

use cid::Cid;
use filecoin_spec::{Commitment, RegisteredSealProof, Ticket};
use fvm_shared::piece::PaddedPieceSize;
use serde::{de::Unexpected, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{
    base64::{Base64, Standard},
    formats::Padded,
    serde_as,
};
use url::Url;

use super::chain::epoch::ChainEpoch;

pub mod policy;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SectorLocation {
    pub local: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Sector {
    pub miner: u64,
    pub number: u64,
}

#[derive(Serialize, Debug)]
pub enum LotusState {
    Packing,
    GetTicket,
    PreCommitting,
    SubmitCommit,
    Proving,
    Available,
}

#[derive(Serialize)]
struct Base64Cid {
    pub cid: Cid,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Piece {
    #[serde(rename(serialize = "PieceCID"))]
    #[serde(serialize_with = "cid_serialize")]
    pub cid: Cid,

    pub size: PaddedPieceSize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SectorPiece {
    pub piece: Piece,
}

pub fn cid_serialize<S>(cid: &Cid, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = s.serialize_map(Some(1))?;
    map.serialize_entry("/", cid.to_string().as_str())?;
    map.end()
}

pub fn cid_deserialize<'de, D>(deserializer: D) -> Result<Cid, D::Error>
where
    D: Deserializer<'de>,
{
    let mut cid: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    tracing::info!("Map {:?}", cid);
    let cid = cid.remove("/").unwrap();
    let cid = Cid::from_str(&cid).unwrap();

    Ok(cid)
}

fn proof_type_serialize<S>(proof: &RegisteredSealProof, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match proof {
        RegisteredSealProof::StackedDrg2KiBV1 => s.serialize_u64(0),
        RegisteredSealProof::StackedDrg8MiBV1 => s.serialize_u64(1),
        RegisteredSealProof::StackedDrg512MiBV1 => s.serialize_u64(2),
        RegisteredSealProof::StackedDrg32GiBV1 => s.serialize_u64(3),
        RegisteredSealProof::StackedDrg64GiBV1 => s.serialize_u64(4),
        RegisteredSealProof::StackedDrg2KiBV1_1 => s.serialize_u64(5),
        RegisteredSealProof::StackedDrg8MiBV1_1 => s.serialize_u64(6),
        RegisteredSealProof::StackedDrg512MiBV1_1 => s.serialize_u64(7),
        RegisteredSealProof::StackedDrg32GiBV1_1 => s.serialize_u64(8),
        RegisteredSealProof::StackedDrg64GiBV1_1 => s.serialize_u64(9),
    }
}

pub fn proof_type_deserialize<'de, D>(deserializer: D) -> Result<RegisteredSealProof, D::Error>
where
    D: Deserializer<'de>,
{
    let id = u64::deserialize(deserializer)?;
    match id {
        0 => Ok(RegisteredSealProof::StackedDrg2KiBV1),
        1 => Ok(RegisteredSealProof::StackedDrg8MiBV1),
        2 => Ok(RegisteredSealProof::StackedDrg512MiBV1),
        3 => Ok(RegisteredSealProof::StackedDrg32GiBV1),
        4 => Ok(RegisteredSealProof::StackedDrg64GiBV1),
        5 => Ok(RegisteredSealProof::StackedDrg2KiBV1_1),
        6 => Ok(RegisteredSealProof::StackedDrg8MiBV1_1),
        7 => Ok(RegisteredSealProof::StackedDrg512MiBV1_1),
        8 => Ok(RegisteredSealProof::StackedDrg32GiBV1_1),
        9 => Ok(RegisteredSealProof::StackedDrg64GiBV1_1),
        v => Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(v),
            &"[0-9]",
        )),
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub state: LotusState,
    pub sector: Sector,

    #[serde(rename(serialize = "Type"))]
    #[serde(serialize_with = "proof_type_serialize")]
    pub proof_type: RegisteredSealProof,

    pub pieces: Vec<SectorPiece>,
    pub ticket_value: Ticket,
    pub ticket_epoch: ChainEpoch,

    pub pre_commit1_out: Commitment,

    #[serde(serialize_with = "cid_serialize")]
    pub comm_d: Cid,

    #[serde(serialize_with = "cid_serialize")]
    pub comm_r: Cid,

    pub data_unsealed: SectorLocation,

    pub data_sealed: SectorLocation,
    pub data_cache: SectorLocation,

    pub remote_commit1_endpoint: Url,
    pub remote_commit2_endpoint: Url,
    pub remote_sealing_done_endpoint: Url,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteC1Request {
    pub ticket: Ticket,
    pub seed: Ticket,

    #[serde(deserialize_with = "cid_deserialize")]
    pub unsealed: Cid,

    #[serde(deserialize_with = "cid_deserialize")]
    pub sealed: Cid,

    #[serde(deserialize_with = "proof_type_deserialize")]
    #[serde(rename(deserialize = "ProofType"))]
    pub proof: RegisteredSealProof,
}

#[serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteC2Request {
    pub sector: Sector,

    #[serde(deserialize_with = "proof_type_deserialize")]
    #[serde(rename(deserialize = "ProofType"))]
    pub proof: RegisteredSealProof,

    #[serde_as(as = "Base64<Standard, Padded>")]
    pub commit1_out: Vec<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DoneRequest {
    #[serde(deserialize_with = "cid_deserialize")]
    pub commit_message: Cid,

    pub state: String,

    pub successful: bool,
}
