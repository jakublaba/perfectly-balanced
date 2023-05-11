use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct ConfigProperties {
    #[serde(default = "default_address")]
    pub(crate) address: String,
    pub(crate) backends: Vec<String>,
    // TODO - add load balancing strategy property once they're implemented
}

fn default_address() -> String {
    String::from("localhost:8080")
}
