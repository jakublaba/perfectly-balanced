use std::sync::{Arc, Mutex};

use crate::load_balancing::strategy::Strategy;

pub(crate) struct RoundRobinStrategy {
    pub(crate) receiver_addresses: Vec<String>,
    counter: Arc<Mutex<u32>>,
}

impl RoundRobinStrategy {
    pub fn new(receiver_addresses: Vec<String>) -> Self {
        RoundRobinStrategy {
            receiver_addresses,
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

impl Strategy for RoundRobinStrategy {
    fn choose_receiver_ip(&self) -> String {
        let mut count = self.counter.lock().unwrap();
        let ip = self.receiver_addresses[*count as usize].clone();
        *count = (*count + 1) % self.receiver_addresses.len() as u32;
        ip
    }
}

#[cfg(test)]
mod test {
    use crate::load_balancing::round_robin::RoundRobinStrategy;
    use crate::load_balancing::strategy::Strategy;

    #[test]
    fn should_return_subsequent_ips() {
        let rr_strategy = RoundRobinStrategy::new(vec![
            String::from("127.0.0.1:81"),
            String::from("127.0.0.1:82"),
            String::from("127.0.0.1:83"),
        ]);

        let l = rr_strategy.receiver_addresses.len();
        for i in 0..100 {
            let expected_ip = rr_strategy.receiver_addresses[i % l].clone();
            let actual_ip = rr_strategy.choose_receiver_ip();
            assert_eq!(expected_ip, actual_ip);
        }
    }
}
