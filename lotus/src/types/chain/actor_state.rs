use crate::types::chain::address::Address;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ActorState {
    pub Balance: String,
    pub Address: Address,
}
