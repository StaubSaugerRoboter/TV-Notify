mod mqtt;
mod command;
mod config;

use log::{error, info, set_max_level, LevelFilter};
use std::cmp::min;
use std::{thread, time};

fn set_up_logger() {
	if systemd_journal_logger::connected_to_journal() {
		systemd_journal_logger::init_with_extra_fields(vec![(
			"VERSION",
			env!("CARGO_PKG_VERSION"),
		)])
			.unwrap();
	} else {
		env_logger::init();
	}
	set_max_level(LevelFilter::Info);
}

fn main() {
	set_up_logger();
	let config = config::config_test().unwrap();


	let mut wait_secs = 2;
	let mut last_state = "start";
	let mut cli = mqtt::create_mqtt(&config.host);

	loop {
		thread::sleep(time::Duration::from_secs(wait_secs));
		let command_output = match command::get_command(&config.command, &config.args) {
			Ok(val) => val,
			Err(e) => {
				error!("Could not get modetest: {:?} ", e);
				wait_secs = min(wait_secs * 2, config.max_timeout);
				error!("Setting wait to: {}", wait_secs);
				last_state = "error";
				continue;
			}
		};

		let command_result = command::parse_command(&command_output, &config.regex, &config.true_strings);
		if let Some(state) = command_result {
			let payload = if state { "on" } else { "off" };
			if payload != last_state {
				let mut error = false;
				if let Err(e) = &cli {
					error!("Could not connect to server: {:?}", e);
					error = true;
				} else if let Err(e) = mqtt::publish(cli.as_mut().unwrap(), &config.topic, payload) {
					error!("Error publishing: {:?}", e);
					error = true;
				}

				if error {
					wait_secs = min(wait_secs * 2, config.max_timeout);
					error!("Setting wait to: {}", wait_secs);
					cli = mqtt::create_mqtt(&config.host);
					last_state = "error";
					continue;
				}

				info!("Display state changed from {} to {}", last_state, payload);
				last_state = payload;
				wait_secs = 2;
			}
		} else {
			error!("Could not get display status from modetest!");
			info!("Dpms: {:?}", command_result);
			wait_secs = std::cmp::min(wait_secs * 2, config.max_timeout);
			last_state = "error";
			error!("Setting wait to: {}", wait_secs);
		}
	}
}

