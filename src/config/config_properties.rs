use serde::{Deserialize, Serialize};

use crate::load_balancing::StrategyOption;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct ConfigProperties {
    #[serde(default = "default_address")]
    pub(crate) address: String,
    #[serde(default = "default_strategy")]
    pub(crate) strategy: StrategyOption,
    pub(crate) receiver_addresses: Vec<String>,
}

fn default_address() -> String {
    String::from("localhost:8080")
}

fn default_strategy() -> StrategyOption {
    StrategyOption::Random
}
