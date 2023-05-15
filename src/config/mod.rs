use std::fs::File;
use std::io::BufReader;

use crate::config::config_properties::ConfigProperties;

pub(crate) mod config_properties;

pub(crate) fn configure(config_file_path: &str) -> Result<ConfigProperties, serde_json::Error> {
    let file = File::open(config_file_path).unwrap();
    serde_json::from_reader(BufReader::new(file))
}

#[cfg(test)]
mod tests {
    use crate::config::config_properties::ConfigProperties;
    use crate::config::configure;
    use crate::load_balancing::strategy::StrategyOption;

    #[test]
    fn should_load_props_if_all_present() {
        let test_file = "src/config/test_resources/all_props.json";
        let expected_props = ConfigProperties {
            address: String::from("127.0.0.1:80"),
            strategy: StrategyOption::RoundRobin,
            receiver_addresses: vec![
                String::from("127.0.0.1:81"),
                String::from("127.0.0.1:82"),
                String::from("127.0.0.1:83"),
                String::from("127.0.0.1:84"),
            ],
        };
        let actual_props = configure(test_file).unwrap();
        assert_eq!(expected_props, actual_props)
    }

    #[test]
    fn should_load_default_address_if_not_present() {
        let test_file = "src/config/test_resources/no_address.json";
        let expected_props = ConfigProperties {
            address: String::from("localhost:8080"),
            strategy: StrategyOption::IpHash,
            receiver_addresses: vec![
                String::from("127.0.0.1:81"),
                String::from("127.0.0.1:82"),
                String::from("127.0.0.1:83"),
                String::from("127.0.0.1:84"),
            ],
        };
        let actual_props = configure(test_file).unwrap();
        assert_eq!(expected_props, actual_props);
    }

    #[test]
    fn should_load_default_strategy_if_not_present() {
        let test_file = "src/config/test_resources/no_strategy.json";
        let expected_props = ConfigProperties {
            address: String::from("127.0.0.1:80"),
            strategy: StrategyOption::Random,
            receiver_addresses: vec![
                String::from("127.0.0.1:81"),
                String::from("127.0.0.1:82"),
                String::from("127.0.0.1:83"),
                String::from("127.0.0.1:84"),
            ],
        };
        let actual_props = configure(test_file).unwrap();
        assert_eq!(expected_props, actual_props);
    }
}
