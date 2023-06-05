use crate::config::config_properties::ConfigProperties;
use crate::load_balancing::ip_hash::IpHashStrategy;
use crate::load_balancing::random::RandomStrategy;
use crate::load_balancing::round_robin::RoundRobinStrategy;
use crate::load_balancing::strategy::Strategy;
use crate::load_balancing::strategy::StrategyOption::{IpHash, Random, RoundRobin};

pub(crate) mod ip_hash;
pub(crate) mod random;
pub(crate) mod round_robin;
pub(crate) mod strategy;

pub(crate) struct LoadBalancer {
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
            receiver_addresses: config.receiver_addresses,
            strategy: match config.strategy {
                Random => Box::new(RandomStrategy::new()),
                RoundRobin => Box::new(RoundRobinStrategy::new()),
                IpHash => Box::new(IpHashStrategy::new()),
            },
        }
    }

    pub(crate) fn get_receiver_ip(&self, requester_ip: String) -> String {
        let ctx = RequestContext {
            requester_ip,
            receiver_ips: &self.receiver_addresses,
        };

        log::info!("Packet will be forwarded from {}", ctx.requester_ip.clone());
        let receiver_ip = self.strategy.choose_receiver_ip(&ctx);
        log::info!("Packets fill be forwarded to {}", receiver_ip.clone());
        return receiver_ip;
    }
}
