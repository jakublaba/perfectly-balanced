use serde::Deserialize;

use async_trait::async_trait;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum StrategyOption {
    Random,
    RoundRobin,
    IpHash,
}

#[async_trait]
pub(crate) trait Strategy: Send + Sync {
    fn choose_receiver_ip(&self) -> String;
}
