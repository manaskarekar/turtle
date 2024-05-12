mod tokenizer;

use std::{error::Error, io};
use turtle::*;

fn main_loop() -> Result<(), Box<dyn Error>> {
	static PROMPT_SYMBOL: &str = "$ ";

	let mut buffer = String::new();
	loop {
		eprint!("{PROMPT_SYMBOL}");
		buffer.clear();

		let stdin = io::stdin();
		let bytes_read = stdin.read_line(&mut buffer)?;

		if bytes_read == 0 {	//Handle the EOF state
			eprintln!();
			std::process::exit(0);
		}

		match tokenizer::tokenizer(&buffer) {
			Ok(tokens) => {
				let (valid, message) = is_user_input_valid(&tokens);
				if valid {
					if BUILTIN_COMMANDS.contains(&tokens[0].as_str()) {
						match handle_builtin_commands(&tokens) {
							Ok(command_executed) => {
								if command_executed { continue; }
							},
							Err(err) => {
								eprintln!("{}", err);
								continue;
							 },
						}
					} else {
						match build_command(tokens) {
							Ok((command, args)) => {
								if let Err(err) = launch_command(command, &args) {
									eprintln!("{}", err);
								}
							},
							Err(err) => eprintln!("{}", err),
						}
					}
				} else if !message.is_empty() {
					eprintln!("error: unsupported input: {}", message)
				}
			},
			Err(err) => eprintln!("{}", err),
		}
	}
}

fn main() {
	loop {
		if let Err(err) = main_loop() {	//NOTE: We're not really returning any Errors here yet, but we could
			eprintln!("{}", err);
		}
	}
}
