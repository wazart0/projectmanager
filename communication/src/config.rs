use std::collections::HashMap;

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct Config {
    pub config_id: i64,
    pub config_key: String,
    pub config_value: Option<String>,
    pub description: Option<String>,
}

impl Config {
    /// Converts a vector of Config objects into a HashMap where the key is the config_key
    /// and the value is the Config object itself.
    pub fn into_hashmap(configs: Vec<Config>) -> HashMap<String, Config> {
        configs
            .into_iter()
            .map(|config| (config.config_key.clone(), config))
            .collect()
    }

    /// Retrieves the config_value as a string, returning None if the value is None
    pub fn value_as_string(&self) -> Option<String> {
        self.config_value.clone()
    }

    /// Attempts to parse the config_value as a specific type that implements FromStr
    pub fn value_as<T>(&self) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.config_value.as_ref().and_then(|v| v.parse::<T>().ok())
    }
}
