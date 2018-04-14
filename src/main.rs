#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
extern crate ctrlc;
extern crate rustyline;
extern crate teko;

use clap::{Arg, ArgGroup, App};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
	let matches = App::new("Teko")
		.version("0.3.0-alpha")
		.author("Kevin Robert Stravers <kefin@stravers.net>")
		.about("Interpreter for the Teko programming language")
		.arg(
			Arg::with_name("v")
				.short("v")
				.multiple(true)
				.help("Sets the level of verbosity"),
		)
		.arg(
			Arg::with_name("expression")
				.short("e")
				.long("expression")
				.multiple(false)
				.takes_value(true)
				.conflicts_with("INPUT")
				.help("Evaluate an expression"),
		)
		.arg(
			Arg::with_name("INPUT")
				.help("File or expression to run")
				.conflicts_with("expression")
				.index(1),
		)
		.get_matches();

	let verbosity = matches.occurrences_of("v");
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
	use std::env;

	use teko::data_structures::*;
	use teko::interpret::*;
	use teko::parse::*;

	const HISTORY: &'static [&str] = &[".config", "teko-history"];
	let home = if let Some(mut path) = env::home_dir() {
		for elem in HISTORY {
			path.push(elem);
		}
		Some(path)
	} else {
		None
	};

	let mut rl = Editor::<()>::new();
	if let Some(ref home) = home {
		if rl.load_history(home).is_err() {
			println![
				"Unable to load history from {:?} \
				 if this is your first time running Teko you can ignore this message",
				home
			];
		}
	}

	let mut env = initialize_environment_with_standard_library();
	let mut inputline = 1;
	let mut parser = ParseState::from("tty");
	parser.current_read_position.line = inputline;

	loop {
		let readline = rl.readline(if is_empty(&parser) { "> " } else { "   " });
		match readline {
			Ok(line) => {
				inputline += 1;
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
				parser.stack = vec![vec![]];
				parser.token = String::new();
				parser.error = None;
				parser.unmatched_opening_parentheses = vec![];
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
	}
	if let Some(ref home) = home {
		if let Err(err) = rl.save_history(home) {
			println!["Unable to save history in {:?}: {:#?}", home, err];
		}
	}
}
