
use cosmwasm_std::CustomQuery;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QuadrateQuery {
        ActualPrice {},
}

impl CustomQuery for QuadrateQuery {}