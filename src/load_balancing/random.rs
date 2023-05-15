use crate::load_balancing::strategy::Strategy;

pub(crate) struct RandomStrategy {
    pub(crate) receiver_addresses: Vec<String>,
}

impl RandomStrategy {
    pub fn new(receiver_addresses: Vec<String>) -> Self {
        RandomStrategy { receiver_addresses }
    }
}

impl Strategy for RandomStrategy {
    fn choose_receiver_ip(&self) -> String {
        String::from("random")
    }
}
