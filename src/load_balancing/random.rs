use rand::Rng;

use crate::load_balancing::RequestContext;
use async_trait::async_trait;

use crate::load_balancing::strategy::Strategy;

pub(crate) struct RandomStrategy;

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy {}
    }
}

#[async_trait]
impl Strategy for RandomStrategy {
    fn choose_receiver_ip<'l>(&self, ctx: &'l RequestContext) -> String {
        let idx = rand::thread_rng().gen_range(0..ctx.receiver_ips.len());
        ctx.receiver_ips[idx].clone()
    }
}

#[cfg(test)]
mod test {
    use crate::load_balancing::random::RandomStrategy;
    use crate::load_balancing::strategy::Strategy;
    use crate::load_balancing::RequestContext;

    #[test]
    fn should_generate_numbers_in_range() {
        let rand_strategy = RandomStrategy::new();
        let ctx = RequestContext {
            requester_ip: String::from("1.2.3.4:5"),
            receiver_ips: &vec![
                String::from("127.0.0.1:81"),
                String::from("127.0.0.1:82"),
                String::from("127.0.0.1:83"),
            ],
        };

        for _i in 0..100 {
            let ip = rand_strategy.choose_receiver_ip(&ctx);
            assert!(ctx.receiver_ips.contains(&ip));
        }
    }
}
