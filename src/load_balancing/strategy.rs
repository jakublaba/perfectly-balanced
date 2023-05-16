use serde::Deserialize;

use crate::load_balancing::RequestContext;
use async_trait::async_trait;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum StrategyOption {
    Random,
    RoundRobin,
    IpHash,
}

#[async_trait]
pub(crate) trait Strategy: Send + Sync {
    fn choose_receiver_ip<'l>(&self, ctx: &'l RequestContext) -> String;
}
