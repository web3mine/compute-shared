pub mod client;
pub mod config;
pub mod helpers;
pub mod serialization;
pub mod types;

use crate::client::LotusClient;

use crate::types::api::RPCRequest;
use crate::types::chain::actor_state::ActorState;
use crate::types::chain::chain_head::ChainHead;
use crate::types::chain::cid::CID;
use crate::types::chain::message::Message;
use crate::types::chain::miner_info::MinerInfo;
use crate::types::chain::network::NetworkVersion;
use crate::types::chain::randomness::DomainSeparationTag;
use crate::types::chain::sector::SectorPreCommitOnChainInfo;
use crate::types::state::state::State;
use async_trait::async_trait;
use config::LotusConfig;
use mockall::automock;
use reqwest::StatusCode;
use serde_json::Value::Null;
use serde_json::{json, Value};
use types::chain::epoch::ChainEpoch;
use types::chain::randomness::{Entropy, Randomness};

pub use fvm_shared::address::Address;
use types::miner::Meta;
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("StatusCode: {0}, Body: {1}")]
    Lotus(StatusCode, String),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    Decode(#[from] base64::DecodeError),
}

type Result<T> = std::result::Result<T, Error>;

#[automock]
#[async_trait]
pub trait Lotus {
    async fn chain_head(&self) -> Result<ChainHead>;
    async fn chain_get_tip_set_by_height(&self, height: i64) -> Result<ChainHead>;
    async fn state_compute(
        &self,
        height: i64,
        messages: Vec<Message>,
        tip_sets: Vec<CID>,
    ) -> Result<State>;

    async fn chain_get_messages_in_tipset(&self, block_cid: CID) -> Result<Vec<Message>>;
    async fn state_decode_params(&self, to: String, method: i64, params: String) -> Result<Value>;
    async fn state_lookup_robust_address(&self, address: String, ts: Option<CID>)
        -> Result<String>;
    async fn state_get_randomness_from_beacon(
        &self,
        personalization: DomainSeparationTag,
        target_epoch: ChainEpoch,
        entropy: &Entropy,
        tipset_key: Vec<CID>,
    ) -> Result<Randomness>;
    async fn state_get_randomness_from_tickets(
        &self,
        personalization: DomainSeparationTag,
        rand_epoch: ChainEpoch,
        entropy: Entropy,
        tipset_key: Vec<CID>,
    ) -> Result<Randomness>;
    async fn state_lookup_id(&self, address: String, ts: Option<CID>) -> Result<String>;
    async fn state_get_actor(&self, address: String, ts: Option<CID>) -> Result<ActorState>;
    async fn chain_get_message(&self, message_cid: CID) -> Result<Message>;
    async fn state_miner_info(&self, address: String, ts: Option<CID>) -> Result<MinerInfo>;
    async fn state_network_version(&self, ts: Option<CID>) -> Result<NetworkVersion>;
    async fn state_sector_precommit_info(
        &self,
        address: String,
        sector_number: u64,
        tipset_key: Vec<CID>,
    ) -> Result<SectorPreCommitOnChainInfo>;
    async fn sector_receive(&self, meta: Meta) -> Result<()>;
}

#[async_trait]
impl Lotus for LotusClient {
    async fn chain_head(&self) -> Result<ChainHead> {
        self.send("ChainHead".to_string(), vec![]).await
    }

    async fn chain_get_tip_set_by_height(&self, height: i64) -> Result<ChainHead> {
        self.send(
            "ChainGetTipSetByHeight".to_string(),
            vec![Value::from(height), Null],
        )
        .await
    }

    async fn state_compute(
        &self,
        height: i64,
        messages: Vec<Message>,
        tip_sets: Vec<CID>,
    ) -> Result<State> {
        self.send(
            "StateCompute".to_string(),
            vec![Value::from(height), json!(messages), json!(tip_sets)],
        )
        .await
    }

    async fn chain_get_messages_in_tipset(&self, block_cid: CID) -> Result<Vec<Message>> {
        self.send(
            "ChainGetMessagesInTipset".to_string(),
            vec![json!(vec![block_cid])],
        )
        .await
    }

    async fn state_decode_params(&self, to: String, method: i64, params: String) -> Result<Value> {
        self.send(
            "StateDecodeParams".to_string(),
            vec![json!(to), json!(method), json!(params), Null],
        )
        .await
    }

    async fn state_lookup_robust_address(
        &self,
        address: String,
        ts: Option<CID>,
    ) -> Result<String> {
        self.send(
            "StateLookupRobustAddress".to_string(),
            vec![json!(address), json!(vec![ts])],
        )
        .await
    }
    async fn state_get_randomness_from_beacon(
        &self,
        personalization: DomainSeparationTag,
        target_epoch: ChainEpoch,
        entropy: &Entropy,
        tipset_key: Vec<CID>,
    ) -> Result<Randomness> {
        let res: String = self
            .send(
                "StateGetRandomnessFromBeacon".to_string(),
                vec![
                    json!(personalization.to_u8()),
                    json!(target_epoch),
                    json!(entropy),
                    json!(tipset_key),
                ],
            )
            .await?;
        Ok(base64::decode(res)?)
    }

    async fn state_get_randomness_from_tickets(
        &self,
        personalization: DomainSeparationTag,
        rand_epoch: ChainEpoch,
        entropy: Entropy,
        tipset_key: Vec<CID>,
    ) -> Result<Randomness> {
        let res: String = self
            .send(
                "StateGetRandomnessFromTickets".to_string(),
                vec![
                    json!(personalization.to_u8()),
                    json!(rand_epoch),
                    json!(entropy),
                    json!(tipset_key),
                ],
            )
            .await?;
        Ok(base64::decode(res)?)
    }

    async fn state_lookup_id(&self, address: String, ts: Option<CID>) -> Result<String> {
        self.send(
            "StateLookupID".to_string(),
            vec![json!(address), json!(vec![ts])],
        )
        .await
    }

    async fn state_get_actor(&self, address: String, ts: Option<CID>) -> Result<ActorState> {
        self.send(
            "StateGetActor".to_string(),
            vec![json!(address), json!(vec![ts])],
        )
        .await
    }

    async fn chain_get_message(&self, message_cid: CID) -> Result<Message> {
        self.send("ChainGetMessage".to_string(), vec![json!(message_cid)])
            .await
    }

    async fn state_miner_info(&self, address: String, ts: Option<CID>) -> Result<MinerInfo> {
        self.send(
            "StateMinerInfo".to_string(),
            vec![json!(address), json!(vec![ts])],
        )
        .await
    }

    async fn state_network_version(&self, ts: Option<CID>) -> Result<NetworkVersion> {
        self.send("StateNetworkVersion".to_string(), vec![json!(vec![ts])])
            .await
    }

    async fn state_sector_precommit_info(
        &self,
        address: String,
        sector_number: u64,
        tipset_key: Vec<CID>,
    ) -> Result<SectorPreCommitOnChainInfo> {
        self.send(
            "StateSectorPreCommitInfo".to_string(),
            vec![json!(address), json!(sector_number), json!(tipset_key)],
        )
        .await
    }

    async fn sector_receive(&self, meta: Meta) -> Result<()> {
        self.send_no_reponse_body("SectorReceive".to_string(), vec![json!(meta)])
            .await
    }
}

pub fn generate_lotus_client(lotus_client_uri: &Url, lotus_token: &Option<String>) -> LotusClient {
    let host_url = lotus_client_uri.join("/rpc/v0").unwrap();
    let content_type = Some("text/plain".to_string());
    LotusClient::init(LotusConfig::new(
        host_url.to_string(),
        content_type,
        lotus_token.clone(),
    ))
}
