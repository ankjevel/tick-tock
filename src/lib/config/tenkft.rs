use config::Config;

#[derive(Debug, Deserialize, Serialize)]
pub struct TenKFeet {
    pub root: String,
    pub token: String,
}

impl TenKFeet {
    pub fn default_config() -> TenKFeet {
        TenKFeet {
            root: String::new(),
            token: String::new(),
        }
    }

    pub fn merge_with_config(&self, config: &mut Config, prefix: &str) {
        let key = prefix.to_string() + "root";
        let _ = config.set_default(&key, self.root.to_string());

        let key = prefix.to_string() + "token";
        let _ = config.set_default(&key, self.token.to_string());
    }
}
