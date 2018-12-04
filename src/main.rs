#![feature(box_patterns)]

mod loc;
mod token;
mod error;
mod lexer;
mod node;
mod parser;
mod cpu;

use lexer::Lexer;
use token::TokenId;

use parser::parse;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use cpu::{Opcode, Inst};

use node::Node;


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
							let program_result = parse(source);

							println!("{:?}", program_result);

							match program_result {
								Err(_) => (),
								Ok(mut program) => {
									let program_gen_result = program.gen();

									println!("{:?}", program_gen_result);

									match program_gen_result {
										None => (),
										Some(raw_insts) => for inst in raw_insts {
											println!("{:032b}", inst);
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
