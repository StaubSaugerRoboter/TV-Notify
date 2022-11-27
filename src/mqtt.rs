use paho_mqtt as mqtt;
use std::time::Duration;

pub fn create_mqtt(host: &str) -> mqtt::Result<mqtt::Client> {
	let mut cli = mqtt::Client::new(host)?;
	cli.set_timeout(Duration::from_secs(5));
	cli.connect(None)?;
	Ok(cli)
}

pub fn publish(cli: &mut mqtt::Client, topic: &str, payload: &str) -> mqtt::Result<()> {
	let msg = mqtt::MessageBuilder::new()
		.topic(topic)
		.payload(payload)
		.qos(1)
		.retained(true)
		.finalize();
	cli.publish(msg)
}
