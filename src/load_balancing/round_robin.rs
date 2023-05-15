use crate::load_balancing::strategy::Strategy;

pub(crate) struct RoundRobinStrategy {
    pub(crate) receiver_addresses: Vec<String>,
    counter: i32,
}

impl RoundRobinStrategy {
    pub fn new(receiver_addresses: Vec<String>) -> Self {
        RoundRobinStrategy {
            receiver_addresses,
            counter: 0,
        }
    }
}

impl Strategy for RoundRobinStrategy {
    fn choose_receiver_ip(&self) -> String {
        String::from("round robin")
    }
}
