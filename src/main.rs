extern crate clap;
extern crate ctrlc;
extern crate rustyline;
extern crate teko;

use clap::{Arg, ArgGroup, App};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
	let matches = App::new("Teko")
		.version("0.1.0")
		.author("Kevin Robert Stravers <macocio@gmail.com>")
		.about("Interpreter for the Teko programming language")
		.arg(
			Arg::with_name("INPUT")
				.help("File or expression to run")
				.index(1),
		)
		.arg(
			Arg::with_name("expression")
				.short("e")
				.multiple(false)
				.takes_value(true)
				.help("Evaluate an expression"),
		)
		.arg(
			Arg::with_name("v")
				.short("v")
				.multiple(true)
				.help("Sets the level of verbosity"),
		)
		.group(
			ArgGroup::with_name("either_e_or_file")
				.args(&["expression", "INPUT"])
				.conflicts_with("INPUT"),
		)
		.get_matches();

	match matches.occurrences_of("v") {
		0 => {}
		1 => println!("Verbose level 1"),
		2 | _ => println!("Verbose level 2"),
	}

	if let Some(input) = matches.value_of("INPUT") {
		let tree = teko::parse::parse_file(input);
		if let Ok(tree) = tree {
			teko::interpret::interpret(tree);
		} else {
			println!["parse error {:#?}", tree];
		}
	} else if let Some(input) = matches.value_of("expression") {
		let tree = teko::parse::parse_string(input);
		if let Ok(tree) = tree {
			let env = teko::interpret::interpret(tree);
			println!["{}", env.result];
		} else {
			println!["parse error {:#?}", tree];
		}
	} else {
		from_terminal();
	}
}

fn from_terminal() {
	use teko::data_structures::*;
	use teko::interpret::*;
	use teko::parse::*;

	const HISTORY: &str = "~/.config/teko-history";

	let mut rl = Editor::<()>::new();
	if let Err(_) = rl.load_history(HISTORY) {
		// No previous history
	}

	let mut env = initialize_environment_with_standard_library();
	let mut inputline = 1;
	let mut parser = ParseState::from("tty");
	parser.current_read_position.line = inputline;

	loop {
		let readline = rl.readline(if is_empty(&parser) { ">> " } else { "   " });
		match readline {
			Ok(line) => {
				rl.add_history_entry(&line);
				for ch in line.chars() {
					if let Err(state) = parse_character(ch, &mut parser) {
						println!["{:#?}", state];
						parser = ParseState::from("tty");
						parser.current_read_position.line = inputline;
						break;
					}
					if is_ready_to_finish(&parser) {
						let result = finish_parsing_characters(parser);
						parser = ParseState::from("tty");
						parser.current_read_position.line = inputline;
						match result {
							Ok(tree) => {
								env = eval(tree, env);
								println!["{}", env.result];
							}
							Err(state) => {
								println!["{:#?}", state];
								break;
							}
						}
					}
				}
				let _ = parse_character('\n', &mut parser);
				if is_ready_to_finish(&parser) {
					let result = finish_parsing_characters(parser);
					parser = ParseState::from("tty");
					parser.current_read_position.line = inputline;
					match result {
						Ok(tree) => {
							parser = ParseState::from("tty");
							parser.current_read_position.line = inputline;
							env = eval(tree, env);
							println!["{}", env.result];
						}
						Err(state) => {
							println!["{:#?}", state];
						}
					}
				}
			}
			Err(ReadlineError::Interrupted) => {
				println!("^C");
			}
			Err(ReadlineError::Eof) => {
				println!("^D");
				break;
			}
			Err(err) => {
				println!("Line reading error: {:?}", err);
				break;
			}
		}
		inputline += 1;
	}
	if let Err(err) = rl.save_history(HISTORY) {
		println!["Unable to save history: {:#?}", err];
	}
}
