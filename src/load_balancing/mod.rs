use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum StrategyOption {
    Random,
    RoundRobin,
    IpHash,
}
