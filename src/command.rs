use regex::Regex;
use std::process::Command;
use std::string::FromUtf8Error;
use log::error;


pub fn parse_command(output: &str, regex: &str, true_strings: &[String]) -> Option<bool> {
    let re: Regex = Regex::new(regex).unwrap();

	let re_result = re.captures(output);
	if let Some(captures) = re_result {
		if true_strings.contains(&captures.get(1).unwrap().as_str().to_string()){
			Some(true)
		} else {
			Some(false)
		}
	} else {
		None
	}
}


pub fn get_command(command: &str, args: &[String]) -> Result<String, FromUtf8Error> {
	let mut command_builder = Command::new(command);

	for arg in args {
		command_builder.arg(arg);
	}

	let output = command_builder.output();

	if let Err(e) = &output {
		error!("Error getting modetest: {:?}", e);
		return Ok("".to_string());
	}

	let modetest = output.unwrap().stdout;
	String::from_utf8(modetest)
}
