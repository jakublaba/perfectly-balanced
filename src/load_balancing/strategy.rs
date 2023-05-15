use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum StrategyOption {
    Random,
    RoundRobin,
    IpHash,
}

pub(crate) trait Strategy {
    fn choose_receiver_ip(&self) -> String;
}
