use rand::Rng;

use async_trait::async_trait;

use crate::load_balancing::strategy::Strategy;

pub(crate) struct RandomStrategy {
    pub(crate) receiver_addresses: Vec<String>,
}

impl RandomStrategy {
    pub fn new(receiver_addresses: Vec<String>) -> Self {
        RandomStrategy { receiver_addresses }
    }
}

#[async_trait]
impl Strategy for RandomStrategy {
    fn choose_receiver_ip(&self) -> String {
        let idx = rand::thread_rng().gen_range(0..self.receiver_addresses.len());
        self.receiver_addresses[idx].clone()
    }
}

#[cfg(test)]
mod test {
    use crate::load_balancing::random::RandomStrategy;
    use crate::load_balancing::strategy::Strategy;

    #[test]
    fn should_generate_numbers_in_range() {
        let rand_strategy = RandomStrategy::new(vec![
            String::from("127.0.0.1:81"),
            String::from("127.0.0.1:82"),
            String::from("127.0.0.1:83"),
        ]);

        for _i in 0..100 {
            let ip = rand_strategy.choose_receiver_ip();
            assert!(rand_strategy.receiver_addresses.contains(&ip));
        }
    }
}
