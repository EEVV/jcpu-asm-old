mod loc;
mod token;
mod error;
mod lexer;
mod node;
mod parser;

use parser::parse;

use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
	match env::args().nth(1) {
		None => println!("usage: jcpu-asm [PATH TO FILE]"),
		Some(x) => {
			let file_maybe = File::open(x);
			match file_maybe {
				Err(_) => println!("invalid file"),
				Ok(mut file) => {
					let mut source = String::new();

					match file.read_to_string(&mut source) {
						Err(_) => println!("failed to read file"),
						Ok(_) => {
							println!("{:?}", parse(source));
						}
					}
				}
			}
		}
	}
}
