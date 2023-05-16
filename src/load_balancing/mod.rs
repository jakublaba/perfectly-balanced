use crate::config::config_properties::ConfigProperties;
use crate::load_balancing::ip_hash::IpHashStrategy;
use crate::load_balancing::random::RandomStrategy;
use crate::load_balancing::round_robin::RoundRobinStrategy;
use crate::load_balancing::strategy::{Strategy, StrategyOption};

pub(crate) mod random;
pub(crate) mod round_robin;
pub(crate) mod ip_hash;
pub(crate) mod strategy;

pub(crate) struct LoadBalancer {
    address: String,
    strategy: Box<dyn Strategy>,
}

impl LoadBalancer {
    pub(crate) fn new(config: ConfigProperties) -> Self {
        LoadBalancer {
            address: config.address,
            strategy: match config.strategy {
                StrategyOption::Random => Box::new(RandomStrategy::new(config.receiver_addresses)),
                StrategyOption::RoundRobin => Box::new(RoundRobinStrategy::new(config.receiver_addresses)),
                StrategyOption::IpHash => Box::new(IpHashStrategy::new(config.receiver_addresses)),
            },
        }
    }
}
