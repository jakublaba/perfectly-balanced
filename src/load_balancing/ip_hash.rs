use crate::load_balancing::strategy::Strategy;

pub(crate) struct IpHashStrategy;

impl IpHashStrategy {
    pub fn new(receiver_addresses: Vec<String>) -> Self {
        IpHashStrategy { receiver_addresses }
    }
}

impl Strategy for IpHashStrategy {
    fn choose_receiver_ip(&self) -> String {
        String::from("ip hash")
    }
}
