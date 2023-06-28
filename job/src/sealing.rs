use crate::{Job, JobType};
use filecoin_spec::{
    ChainEpoch, PieceInfo, RegisteredSealProof, SectorId, StorageProviderId, Ticket,
};
use serde::{Deserialize, Serialize};
use serde_with::{
    base64::{Base64, Standard},
    formats::Padded,
    serde_as,
};

// ****** PC1 **********

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PC1 {
    pub input: PC1Input,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PC1Input {
    pub registered_proof: RegisteredSealProof,
    pub storage_provider_id: StorageProviderId,
    pub sector_id: SectorId,
    pub ticket: Ticket,
    pub ticket_epoch: ChainEpoch,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PC1Output(#[serde_as(as = "Base64<Standard, Padded>")] pub Vec<u8>);

impl AsRef<[u8]> for PC1Output {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<Vec<u8>> for PC1Output {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl Job for PC1 {
    type Input = PC1Input;
    type Output = PC1Output;

    fn job_type() -> JobType {
        JobType::PC1
    }

    fn into_input(self) -> Self::Input {
        self.input
    }
}

impl SealingJob for PC1 {
    fn sector_id(&self) -> SectorId {
        self.input.sector_id
    }

    fn storage_provider_id(&self) -> StorageProviderId {
        self.input.storage_provider_id
    }

    fn registered_proof(&self) -> RegisteredSealProof {
        self.input.registered_proof
    }
}

// ****** PC2 **********

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PC2 {
    pub input: PC2Input,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PC2Output(#[serde_as(as = "Base64<Standard, Padded>")] pub Vec<u8>);

impl AsRef<[u8]> for PC2Output {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<Vec<u8>> for PC2Output {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PC2Input {
    pub pc1_output: PC1Output,
    pub sector_id: SectorId,
    pub storage_provider_id: StorageProviderId,
    pub registered_proof: RegisteredSealProof,
}

impl Job for PC2 {
    type Input = PC2Input;
    type Output = PC2Output;

    fn job_type() -> JobType {
        JobType::PC2
    }

    fn into_input(self) -> Self::Input {
        self.input
    }
}

impl SealingJob for PC2 {
    fn sector_id(&self) -> SectorId {
        self.input.sector_id
    }

    fn storage_provider_id(&self) -> StorageProviderId {
        self.input.storage_provider_id
    }

    fn registered_proof(&self) -> RegisteredSealProof {
        self.input.registered_proof
    }
}

// ****** PC1/PC2 **********

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PC {
    pub input: PC1Input,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct PCOutput(#[serde_as(as = "Base64<Standard, Padded>")] pub Vec<u8>);

impl AsRef<[u8]> for PCOutput {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<Vec<u8>> for PCOutput {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl Job for PC {
    type Input = PC1Input;
    type Output = PCOutput;

    fn job_type() -> JobType {
        JobType::PC
    }

    fn into_input(self) -> Self::Input {
        self.input
    }
}

impl SealingJob for PC {
    fn sector_id(&self) -> SectorId {
        self.input.sector_id
    }

    fn storage_provider_id(&self) -> StorageProviderId {
        self.input.storage_provider_id
    }

    fn registered_proof(&self) -> RegisteredSealProof {
        self.input.registered_proof
    }
}

// ****** C1 **********

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct C1 {
    pub input: C1Input,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct C1Output(#[serde_as(as = "Base64<Standard, Padded>")] pub Vec<u8>);

impl AsRef<[u8]> for C1Output {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<Vec<u8>> for C1Output {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct C1Input {
    pub pc2_output: PC2Output,

    pub storage_provider_id: StorageProviderId,
    pub sector_id: SectorId,
    pub ticket: Ticket,
    pub seed: Ticket,
    pub piece_infos: Vec<PieceInfo>,
    pub registered_proof: RegisteredSealProof,
}

impl Job for C1 {
    type Input = C1Input;
    type Output = C1Output;

    fn job_type() -> JobType {
        JobType::C1
    }

    fn into_input(self) -> Self::Input {
        self.input
    }
}

impl SealingJob for C1 {
    fn sector_id(&self) -> SectorId {
        self.input.sector_id
    }

    fn storage_provider_id(&self) -> StorageProviderId {
        self.input.storage_provider_id
    }

    fn registered_proof(&self) -> RegisteredSealProof {
        self.input.registered_proof
    }
}

// ****** C2 **********

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct C2 {
    pub input: C2Input,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct C2Input {
    pub c1_output: C1Output,
    pub storage_provider_id: StorageProviderId,
    pub sector_id: SectorId,
    pub registered_proof: RegisteredSealProof,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "test-utils", derive(PartialEq, Eq, Clone))]
pub struct C2Output(#[serde_as(as = "Base64<Standard, Padded>")] pub Vec<u8>);

impl AsRef<[u8]> for C2Output {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<Vec<u8>> for C2Output {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

pub trait SealingJob: Job + Send + Sync {
    fn sector_id(&self) -> SectorId;
    fn storage_provider_id(&self) -> StorageProviderId;
    fn registered_proof(&self) -> RegisteredSealProof;
}

impl Job for C2 {
    type Input = C2Input;
    type Output = C2Output;

    fn job_type() -> JobType {
        JobType::C2
    }

    fn into_input(self) -> Self::Input {
        self.input
    }
}

impl SealingJob for C2 {
    fn sector_id(&self) -> SectorId {
        self.input.sector_id
    }
    fn storage_provider_id(&self) -> StorageProviderId {
        self.input.storage_provider_id
    }

    fn registered_proof(&self) -> RegisteredSealProof {
        self.input.registered_proof
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_job_deserialization() {
        let data = r#"
        {
            "registered_proof": "StackedDrg2KiBV1",
            "storage_provider_id": 1111,
            "sector_id": 123123,
            "ticket": "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE=",
            "ticket_epoch": 1100
        }"#;
        let input: PC1Input = serde_json::from_str(data).unwrap();
        let expected_input = PC1Input {
            registered_proof: RegisteredSealProof::StackedDrg2KiBV1,
            storage_provider_id: StorageProviderId(1111),
            sector_id: SectorId(123123),
            ticket: Ticket([1; 32]),
            ticket_epoch: ChainEpoch(1100),
        };
        assert_eq!(input, expected_input);
    }
}
