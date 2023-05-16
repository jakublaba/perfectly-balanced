use std::sync::{Arc, Mutex};

use crate::load_balancing::RequestContext;
use crate::load_balancing::strategy::Strategy;

pub(crate) struct RoundRobinStrategy {
    counter: Arc<Mutex<u32>>,
}

impl RoundRobinStrategy {
    pub fn new() -> Self {
        RoundRobinStrategy {
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

impl Strategy for RoundRobinStrategy {
    fn choose_receiver_ip<'l>(&self, ctx: &'l RequestContext) -> String {
        let mut counter_ptr = self.counter.lock().unwrap();
        let ip = ctx.receiver_ips[*counter_ptr as usize].clone();
        *counter_ptr = (*counter_ptr + 1) % ctx.receiver_ips.len() as u32;
        ip
    }
}

#[cfg(test)]
mod test {
    use crate::load_balancing::round_robin::RoundRobinStrategy;
    use crate::load_balancing::strategy::Strategy;
    use crate::load_balancing::RequestContext;

    #[test]
    fn should_return_subsequent_ips() {
        let rr_strategy = RoundRobinStrategy::new();
        let ctx = RequestContext {
            requester_ip: String::from("1.2.3.4:5"),
            receiver_ips: &vec![
                String::from("127.0.0.1:81"),
                String::from("127.0.0.1:82"),
                String::from("127.0.0.1:83"),
            ],
        };

        let l = ctx.receiver_ips.len();
        for i in 0..100 {
            let expected_ip = ctx.receiver_ips[i % l].clone();
            let actual_ip = rr_strategy.choose_receiver_ip(&ctx);
            assert_eq!(expected_ip, actual_ip);
        }
    }
}
