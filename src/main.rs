// use std::fs;

use env_logger;
use log::{error, info, trace, warn};
use std::cmp::max;
use std::{thread, time};

fn main() {
	env_logger::init();

	let topic = "tv/display/state";
	let host = "tcp://10.0.102.65:1883";
	// let file_content = fs::read_to_string("on").unwrap();

	let mut wait_secs = 2;
	let mut last_state = "start";
	let mut cli = create_mqtt(host);

	loop {
		thread::sleep(time::Duration::from_secs(wait_secs));
		// let modetest = get_modetest().unwrap_or_else(|e| {
		// println!("Error getting modeset: {:?}: ", e);
		// continue;
		// });
		let modetest = match get_modetest() {
			Ok(val) => val,
			Err(e) => {
				error!("Could not get modetest: {:?} ", e);
				wait_secs = max(wait_secs * 2, 180);
				error!("Setting wait to: {}", wait_secs);
				continue;
			}
		};

		// println!("{}", modetest);


		let dpms = get_dpms(&modetest);
		trace!("DPMS: {:?}", dpms);
		if let Some(state) = dpms {
			let payload = if state { "on" } else { "off" };
			if payload != last_state {
				info!("Display state changed from {} to {}", last_state, payload);
				last_state = payload;
				let mut error = false;
				if let Err(e) = &cli {
					error!("Could not connect to server: {:?}", e);
					error = true;
				} else if let Err(e) = publish(cli.as_mut().unwrap(), topic, payload) {
					error!("Error publishing: {:?}", e);
					error = true;
				}

				if error {
					wait_secs = max(wait_secs * 2, 180);
					error!("Setting wait to: {}", wait_secs);
					cli = create_mqtt(host);
					continue;
				}
				wait_secs = 2;
			}
		} else {
			error!("Could not get display status from modetest!");
			info!("Dpms: {:?}", dpms);
			// info!("Modetest: {}", modetest);
			wait_secs = std::cmp::min(wait_secs * 2, 180);
			error!("Setting wait to: {}", wait_secs);
		}
	}
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?msU)HDMI-A-2.*DPMS.*value: (\d)").unwrap();
}

fn get_dpms(modetest: &str) -> Option<bool> {
	// let RE = Regex::new(r"(?msU)HDMI-A-1.*DPMS.*value: (\d)").unwrap();
	let re_result = RE.captures(modetest);
	if let Some(captures) = re_result {
		if captures.get(1).unwrap().as_str() == "0" {
			Some(true)
		} else {
			Some(false)
		}
	} else {
		None
	}
}

use std::process::Command;
use std::string::FromUtf8Error;

fn get_modetest() -> Result<String, FromUtf8Error> {
	let output = Command::new("modetest")
		.output();

	if let Err(e) = &output {
		error!("Error getting modetest: {:?}", e);
		return Ok("".to_string());
	}

	let modetest = output.unwrap().stdout;
	String::from_utf8(modetest)
}

use paho_mqtt as mqtt;
use std::time::Duration;

fn create_mqtt(host: &str) -> mqtt::Result<mqtt::Client> {
	let mut cli = mqtt::Client::new(host)?;
	cli.set_timeout(Duration::from_secs(5));
	cli.connect(None)?;
	Ok(cli)
}

fn publish(cli: &mut mqtt::Client, topic: &str, payload: &str) -> mqtt::Result<()> {
	let msg = mqtt::MessageBuilder::new()
		.topic(topic)
		.payload(payload)
		.qos(1)
		.retained(true)
		.finalize();
	cli.publish(msg)
}
