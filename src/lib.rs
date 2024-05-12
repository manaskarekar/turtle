pub mod tokenizer;	//for accessibility from test/tests.rs

use std::{env, error::Error, fs, io::{self, Write}, os::unix::fs::PermissionsExt, path::{Path, PathBuf}, process::Command};

pub fn is_user_input_valid(tokens: &Vec<String>) -> (bool, &str) {
	// return validity of user input with an optional message

	if tokens.len() == 0 {
		return (false, "");
	} else if tokens.len() > 101 {
		return (false, "Shell doesn't support more than 100 arguments.");
	}
	(true, "")
}

pub fn is_executable(path: PathBuf) -> bool {
	if let Ok(candidate_file) = fs::metadata(path) {
		if candidate_file.is_file() && (candidate_file.permissions().mode() & 0o111) != 0 {
			return true;
		}
	}
	false
}

pub fn lookup_executable(user_command: & str) -> Option<PathBuf> {
	// Perform path resolution in the following order:
	//	see if given path is a direct path to the binary, if found, execute it.
	//	if not, go through all the paths and search for the binary.
	//	Optionally: you can add the current directory to path to allow executing the binary if it's present,
	//		but lets leave it out for now.

	// Currently, it'll reload from PATH for each user input, definitely not optimal, and can be pulled elsewhere to load less often.
	let paths = load_paths_from_env("PATH");

	for path in paths {
		if is_executable(Path::new(&path).join(user_command)) {
			return Some(Path::new(&path).join(user_command));
		}
	}
	None
}

pub fn build_command(mut tokens: Vec<String>) -> Result<(PathBuf, Vec<String>), Box<dyn Error>>{
	//if the command - tokens[0], is an executable, run it.
	//else look up the executable for the given command in the PATH
	let command = if is_executable(PathBuf::from(&tokens[0])) {
		PathBuf::from(&tokens[0])
	} else {
		if let Some(cmd) = lookup_executable(&tokens[0]) {
			cmd
		} else {
			return Err("error: command not found.".into());
		}
	};

	let args = if tokens.len() > 1 {
		tokens.remove(0);
		tokens
	} else {
		Vec::new()
	};

	Ok((command, args))
}

pub fn launch_command(command: PathBuf, args: &[String]) -> Result<(), Box<dyn Error>> {
	let mut cmd = Command::new(&command);

	if !args.is_empty() {
		cmd.args(args);
	}

	match cmd.output() {
		Ok(output) => {
			io::stdout().write_all(&output.stdout)?;
			io::stderr().write_all(&output.stderr)?;
			io::stdout().flush()?;

			if let Some(exit_code) = output.status.code() {
				if exit_code != 0 {
					return Err(format!("error: command exited with code {}", exit_code).into());
				}
			}
		},
		Err(err) => { return Err(format!("{:?} failed: {}", cmd, err).into()); },
	}
	Ok(())
}

pub fn load_paths_from_env(path_var: &str) -> Vec<String>{
	let paths = match env::var_os(path_var) {
		Some(path) => {
			match path.into_string() {
				Ok(path_string) => path_string,
				Err(_) => String::new(),
			}
		},
		None => String::new(),	//No path variables found
	};
	paths.split(":").map(|x| x.to_string()).collect()
}

// These can be pulled out into their own module

pub static BUILTIN_COMMANDS: [&str; 2] = ["cd", "exit"];

pub fn handle_builtin_commands(tokens: &Vec<String>) -> Result<bool, Box<dyn Error>> {
	// NOTE: The result returns Ok(false) if this was not a built-in command.
	//		It returns Ok(true) if it was a built-in command and it executed successfully.

	if tokens[0] == "exit" {
		std::process::exit(0);
	} else if tokens[0] == "cd" {
		let mut path = "/home";	//default to /home if no args specified

		if tokens.len() == 2 {
			let metadata = fs::metadata(&tokens[1]);
			if metadata.is_ok() && metadata?.is_dir() {
				path = &tokens[1];
			} else {
				return Err("error: cd failed - invalid path.".into());
			}
		} else if tokens.len() > 2 {
			return Err("error: cd failed - please specify only one target path.".into());
		}

		if let Err(err) = std::env::set_current_dir(path) {
			return Err(format!("error: cd failed - {}.", err).into());
		}
	} else {
		return Ok(false);
	}

	Ok(true)
}
