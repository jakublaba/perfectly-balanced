use crate::config::config_properties::ConfigProperties;
use crate::load_balancing::ip_hash::IpHashStrategy;
use crate::load_balancing::random::RandomStrategy;
use crate::load_balancing::round_robin::RoundRobinStrategy;
use crate::load_balancing::strategy::StrategyOption::{IpHash, Random, RoundRobin};
use crate::load_balancing::strategy::{Strategy};

pub(crate) mod ip_hash;
pub(crate) mod random;
pub(crate) mod round_robin;
pub(crate) mod strategy;

pub(crate) struct LoadBalancer {
    address: String,
    receiver_addresses: Vec<String>,
    strategy: Box<dyn Strategy>,
}

pub(crate) struct RequestContext<'l> {
    pub(crate) requester_ip: String,
    pub(crate) receiver_ips: &'l Vec<String>,
}

impl LoadBalancer {
    pub(crate) fn new(config: ConfigProperties) -> Self {
        LoadBalancer {
            address: config.address,
            receiver_addresses: config.receiver_addresses,
            strategy: match config.strategy {
                Random => Box::new(RandomStrategy::new()),
                RoundRobin => Box::new(RoundRobinStrategy::new()),
                IpHash => Box::new(IpHashStrategy::new()),
            },
        }
    }
}
