
extern crate turtle;

pub(crate) mod test {
	// NOTE: Similar tests grouped together for conciseness here.
	// TODO: Tests that actually reach out and hook into our shell to interact with std streams,
	//		check validity of executables, and try out quite a few more things.
	// There is a lot more that can be done for both unit and integration testing here, but this is a start.

	use std::path::PathBuf;

	fn vecstr_to_vecstring(v: Vec<&str>) -> Vec<String> {
		// Helper to aid readability in the test cases below
		v.into_iter().map(|x| x.to_string()).collect::<Vec<String>>()
	}

	#[test]
	fn test_is_user_input_valid_true() {
		assert_eq!(turtle::is_user_input_valid(&vec!["ls".to_string()]), (true, ""));
		assert_eq!(turtle::is_user_input_valid(&vec!["token".to_string(); 101]), (true, ""));
	}

	#[test]
	fn test_is_user_input_valid_false() {
		assert_eq!(turtle::is_user_input_valid(&vec![]), (false, ""));
		assert_eq!(turtle::is_user_input_valid(&vec!["token".to_string(); 102]), (false, "Shell doesn't support more than 100 arguments."));
	}

	#[test]
	fn test_tokenizer_success() {
		assert_eq!(turtle::tokenizer::tokenizer(&"printf                 \"Cat's %s\" \"Cradle\"".to_string()).unwrap(),
													vecstr_to_vecstring(vec!["printf", "Cat's %s", "Cradle"]));
		assert_eq!(turtle::tokenizer::tokenizer(&"printf \"Cat's %s\" \"Cradle\"".to_string()).unwrap(),
													vecstr_to_vecstring(vec!["printf", "Cat's %s", "Cradle"]));
		assert_eq!(turtle::tokenizer::tokenizer(&"printf \"\" \"Cat's %s\" \"Cradle\"".to_string()).unwrap(),
													vecstr_to_vecstring(vec!["printf", "", "Cat's %s", "Cradle"]));
		assert_eq!(turtle::tokenizer::tokenizer(&"printf \"\"".to_string()).unwrap(),
													vecstr_to_vecstring(vec!["printf", ""]));
		assert_eq!(turtle::tokenizer::tokenizer(&"printf      ".to_string()).unwrap(),
													vecstr_to_vecstring(vec!["printf"]));
		assert_eq!(turtle::tokenizer::tokenizer(&"printf".to_string()).unwrap(),
													vecstr_to_vecstring(vec!["printf"]));
	}

	#[test]
	fn test_tokenizer_failure() {
		assert!(turtle::tokenizer::tokenizer(&"printf \"Missing `Closing` Quote".to_string()).is_err());
	}

	/*
		NOTE: The following tests are system specific to the build system/test system
			The binaries may be unavailable and/or paths may be different on other systems.
			Adjust your tests accordingly.
	*/

	#[test]
	fn test_lookup_executable() {
		// This test is non-portable.
		let panic_message = "This is a system specific test, check `whereis ls` on the host and tweak the test.";
		assert_eq!(turtle::lookup_executable("ls"), Some(PathBuf::from("/usr/bin/ls")), "{}", panic_message);
		assert!(turtle::lookup_executable("non_existent_bin_30044bf7-e448-44aa-8d87-7ebf85c22fcf").is_none()); //uuidgen generated uuid to avoid accidentally finding such a binary.
	}

	#[test]
	fn test_build_command_success() {
		// This test is non-portable.
		let panic_message = "This is a system specific test, check `whereis ls` on the host and tweak the test.";
		assert_eq!(turtle::build_command(vec!["ls".to_string()]).unwrap(), (PathBuf::from("/usr/bin/ls"), Vec::<String>::new()), "{}", panic_message);
		assert_eq!(turtle::build_command(vec!["ls".to_string()]).unwrap(), (PathBuf::from("/usr/bin/ls"), Vec::<String>::new()), "{}", panic_message);
		assert_eq!(turtle::build_command(vec!["ls".to_string(), "-alh".to_string()]).unwrap(), (PathBuf::from("/usr/bin/ls"), vec!["-alh".to_string()]), "{}", panic_message);
	}

	#[test]
	fn test_handle_builtin_commands() {
		// This test is non-portable.

		//NOTE: This test calls std::process::exit(0), so it literally exits. We can get creative to test it, but leaving it commented for now.
		//assert!(turtle::handle_shell_specific_commands(&vec!["exit".to_string()]).is_ok());

		assert!(turtle::handle_builtin_commands(&vecstr_to_vecstring(vec!["cd", "/"])).is_ok_and(|x| x == true));
		assert!(turtle::handle_builtin_commands(&vecstr_to_vecstring(vec!["cd", "/f44098b6-c618-4ee1-8cf6-25c1b3a23ec4"])).is_err()); //uuidgen generated uuid to avoid accidentally finding such a directory
	}

	#[test]
	fn test_is_executable() {
		// This test is non-portable.
		let panic_message = "This is a system specific test, check `whereis ls` on the host and tweak the test.";
		assert_eq!(turtle::is_executable(PathBuf::from("/usr/bin/ls")), true, "{}", panic_message);
		assert_eq!(turtle::is_executable(PathBuf::from("/usr/bin/non_existent_bin_30044bf7-e448-44aa-8d87-7ebf85c22fcf")), false, "{}", panic_message); //uuidgen generated uuid to avoid accidentally finding such a binary.
	}

	#[test]
	fn test_load_paths_from_env_success() {
		// This test is non-portable.
		let panic_message = "This is a system specific test, `PATH` may not be the path variable of choice, or it might be empty.";
		assert!(!turtle::load_paths_from_env("PATH").is_empty(), "{}", panic_message);
		assert_eq!(turtle::load_paths_from_env("FAKE_PATH"),vec!["".to_string()], "{}", panic_message); //uuidgen generated uuid to avoid accidentally finding such a binary.
	}

	#[test]
	fn test_std_streams() {
		//TODO: Test the streams with a variety of tests. (stdin, stderr, stdout)
	}

}
