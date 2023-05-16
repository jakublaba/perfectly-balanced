use crate::load_balancing::strategy::Strategy;
use crate::load_balancing::RequestContext;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use async_trait::async_trait;

pub(crate) struct IpHashStrategy {
    pub(crate) receiver_addresses: Vec<String>,
}

impl IpHashStrategy {
    pub fn new() -> Self {
        IpHashStrategy {}
    }

     fn calculate_hash<T: Hash>(&self, t: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        hasher.finish()
    }

     fn hash_to_idx(&self, hash: u64, receiver_vector_len: usize) -> usize {
         hash as usize % receiver_vector_len
    }
}

#[async_trait]
impl Strategy for IpHashStrategy {
    fn choose_receiver_ip<'l>(&self, ctx: &'l RequestContext) -> String {
        let requester_hash = self.calculate_hash(&ctx.requester_ip);
        let idx = self.hash_to_idx(requester_hash,ctx.receiver_ips.len());
        ctx.receiver_ips[idx].clone()
    }

}

#[cfg(test)]
mod test {
    use crate::load_balancing::ip_hash::IpHashStrategy;
    use crate::load_balancing::RequestContext;
    use crate::load_balancing::strategy::Strategy;

    #[test]
    fn should_generate_different_hashes() {
        let ip_hash_strategy = IpHashStrategy::new();
        let receiver_addresses = &vec![
           String::from("127.0.0.1:81"),
           String::from("127.0.0.1:82"),
           String::from("127.0.0.1:83")
        ];
        let mut hash_vector = Vec::new();

        for i in receiver_addresses {
            hash_vector.push(ip_hash_strategy.calculate_hash(i));
        }
        assert_ne!(hash_vector[0],hash_vector[1]);
        assert_ne!(hash_vector[0],hash_vector[2]);
        assert_ne!(hash_vector[1],hash_vector[2]);
    }
    #[test]
    fn should_map_to_index_within_bounds() {
        let ip_hash_strategy = IpHashStrategy::new();
        let requester_hash = ip_hash_strategy.calculate_hash(&String::from("127.0.0.1:82"));
        let idx = ip_hash_strategy.hash_to_idx(requester_hash,5);

        assert!(requester_hash > 5);
        assert!(idx < 5);
    }

    #[test]
    fn should_choose_same_ip_for_same_client_and_different_for_different() {
        let ip_hash_strategy = IpHashStrategy::new();
        let ctx = RequestContext {
            requester_ip: String::from("1.2.3.4:5"),
            receiver_ips: &vec![
                String::from("127.0.0.1:81"),
                String::from("127.0.0.1:82"),
                String::from("127.0.0.1:83"),
            ],
        };

        let receiver_ip1 = ip_hash_strategy.choose_receiver_ip(&ctx);
        let receiver_ip2 = ip_hash_strategy.choose_receiver_ip(&ctx);
        assert_eq!(receiver_ip1, receiver_ip2);
    }

    #[test]
    fn should_choose_different_ip_for_different_clients() {
        let ip_hash_strategy = IpHashStrategy::new();
        let ctx1 = RequestContext {
            requester_ip: String::from("1.2.3.4:5"),
            receiver_ips: &vec![
                String::from("127.0.0.1:81"),
                String::from("127.0.0.1:82"),
                String::from("127.0.0.1:83"),
            ],
        };
        let ctx2 = RequestContext {
            requester_ip: String::from("1.2.3.4:6"),
            ..ctx1
        };

        let ip1 = ip_hash_strategy.choose_receiver_ip(&ctx1);
        let ip2 = ip_hash_strategy.choose_receiver_ip(&ctx2);

        assert_ne!(ip1, ip2);
    }
}
