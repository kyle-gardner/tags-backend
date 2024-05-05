use std::env::args;

mod cli;

fn main() {
	handle_args(args().collect());
}

fn handle_args(mut arguments: Vec<String>) {
	// on windows, the first argument is the path to the executable
	// on linux, i believe it doesnt do this, dont know about other OS
	if std::env::consts::OS == "windows" {
		arguments.remove(0);
	}

	if arguments.is_empty() {
		// help 
		cli::help();
	} else {
		if arguments.get(0).unwrap() == "-F" {
			cli::frontend_mode(arguments);
		} else {
			cli::user_mode(arguments);	
		}
	}
}