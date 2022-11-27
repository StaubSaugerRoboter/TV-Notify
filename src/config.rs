use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyConfig {
	pub topic: String,
	pub host: String,
	pub max_timeout: u64,
	pub command: String,
	pub args: Vec<String>,
	pub regex: String,
	pub true_strings: Vec<String>,
}



impl Default for NotifyConfig {
	fn default() -> Self {
		NotifyConfig {
			topic: "tv/display_testing/state".to_string(),
			host: "tcp://10.0.102.65:18830".to_string(),
			max_timeout: 30,
			command: "/opt/vc/bin/vcgencmd".to_string(),
			args: vec!("display_power".to_string()),
			regex: r"(?msU)display_power=(\d)".to_string(),
			true_strings: vec!("1".to_string()),
		}
	}
}

pub fn config_test() -> Result<NotifyConfig, confy::ConfyError> {
	let cfg: NotifyConfig = confy::load_path("/etc/tv-notify/config.toml")?;
	println!("{:#?}", cfg);
	Ok(cfg)
}