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
		None => println!("usage: jcpu-asm [PATH TO INPUT] [PATH TO OUTPUT]"),
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
										Some(raw_insts) => match env::args().nth(2) {
											None => println!("usage: jcpu-asm [PATH TO INPUT] [PATH TO OUTPUT]"),
											Some(x) => {
												let file_maybe = File::create(x);

												match file_maybe {
													Err(_) => println!("failed to create output file"),
													Ok(mut file) => {
														for inst in raw_insts {
															println!("{:032b}", inst);

															file.write(&[
																inst as u8,
																(inst >> 8) as u8,
																(inst >> 16) as u8,
																(inst >> 24) as u8
															]);
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
			}
		}
	}
}
