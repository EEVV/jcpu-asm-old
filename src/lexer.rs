use loc::Loc;
use token::{TokenId, Token};
use error::{ErrorId, Error};


#[derive(Debug)]
pub struct Lexer {
	source: String,
	loc: Loc,
	chr_maybe: Option<char>
}

impl Lexer {
	pub fn new(source: String) -> Lexer {
		Lexer {
			chr_maybe: source.chars().nth(0),
			source: source,
			loc: Loc {
				pos: 0,
				col: 1,
				line: 1
			}
		}
	}

	fn advance(&mut self) {
		if self.chr_maybe.is_some() {
			let chr = self.chr_maybe.unwrap();

			self.loc.col += 1;

			if chr == '\n' {
				self.loc.line += 1;
				self.loc.col = 1;
			}
		}

		self.loc.pos += 1;

		self.chr_maybe = self.source.chars().nth(self.loc.pos);
	}

	fn get_num(&mut self, chr: char) -> isize {
		let mut num: isize = chr.to_digit(10).unwrap() as isize;

		loop {
			match self.chr_maybe {
				None => break,
				Some(chr) => match chr {
					'0' ... '9' => {
						num = num * 10;
						num += chr.to_digit(10).unwrap() as isize;

						self.advance()
					},
					_ => break
				}
			}
		}

		num
	}

	fn get_iden(&mut self, chr: char) -> String {
		let mut iden = chr.to_string();

		loop {
			match self.chr_maybe {
				None => break,
				Some(chr) => match chr {
					'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_' => {
						iden.push(chr);

						self.advance();
					},
					_ => break
				}
			}
		}

		iden
	}

	fn gen_token(&self, token_id: TokenId) -> Result<Token, Error> {
		Ok(Token {
			id: token_id,
			loc: self.loc.clone()
		})
	}

	fn gen_error(&self, error_id: ErrorId, offset: usize) -> Result<Token, Error> {
		Err(Error {
			id: error_id,
			loc: Loc {
				pos: self.loc.pos,
				col: self.loc.col - offset,
				line: self.loc.line
			}
		})
	}

	pub fn token(&mut self) -> Result<Token, Error> {
		match self.chr_maybe {
			None => self.gen_token(TokenId::Eof),
			Some(chr) => match chr {
				' ' => {
					self.advance();

					self.token()
				},
				'#' => {
					self.advance();

					loop {
						match self.chr_maybe {
							None => return self.gen_token(TokenId::Eof),
							Some(chr) => match chr {
								'\n' => {
									self.advance();

									return self.token()
								},
								_ => self.advance()
							}
						}
					}
				},
				'a' ... 'z' => {
					if chr == 'r' {
						self.advance();

						match self.chr_maybe {
							None => self.gen_token(TokenId::Eof),
							Some(chr) => match chr {
								'0' ... '9' => {
									self.advance();

									let num = self.get_num(chr);

									if num > 15 {
										self.gen_error(ErrorId::InvalidReg, 1)
									} else {
										self.gen_token(TokenId::Reg(num))
									}
								},
								_ => {
									self.advance();

									let iden = self.get_iden(chr);

									self.gen_token(TokenId::Iden(iden))
								}		
							}
						}
					} else {
						self.advance();

						let iden = self.get_iden(chr);
						self.gen_token(TokenId::Iden(iden))
					}
				},
				'0' ... '9' => {
					self.advance();

					let num = self.get_num(chr);
					self.gen_token(TokenId::Num(num))
				},
				'!' => {
					self.advance();

					self.gen_token(TokenId::Not)
				},
				'|' => {
					self.advance();

					self.gen_token(TokenId::Or)
				},
				'&' => {
					self.advance();

					self.gen_token(TokenId::And)
				},
				'^' => {
					self.advance();

					self.gen_token(TokenId::Xor)
				},
				'+' => {
					self.advance();

					self.gen_token(TokenId::Add)
				},
				'-' => {
					self.advance();

					match self.chr_maybe {
						None => self.gen_token(TokenId::Sub),
						Some(chr) => if chr == '>' {
							self.advance();

							self.gen_token(TokenId::To)
						} else {
							self.gen_token(TokenId::Sub)
						}
					}
				},
				'*' => {
					self.advance();

					self.gen_token(TokenId::Mul)
				},
				'/' => {
					self.advance();

					self.gen_token(TokenId::Div)
				},
				',' => {
					self.advance();

					self.gen_token(TokenId::Comma)
				},
				'=' => {
					self.advance();

					self.gen_token(TokenId::Eql)
				},
				'<' => {
					self.advance();

					self.gen_token(TokenId::Lt)
				},
				'>' => {
					self.advance();

					self.gen_token(TokenId::Gt)
				},
				'?' => {
					self.advance();

					self.gen_token(TokenId::If)
				},
				'(' => {
					self.advance();

					self.gen_token(TokenId::ParenL)
				},
				')' => {
					self.advance();

					self.gen_token(TokenId::ParenR)
				},
				'[' => {
					self.advance();

					self.gen_token(TokenId::SquareL)
				},
				']' => {
					self.advance();

					self.gen_token(TokenId::SquareR)
				},
				'\t' => {
					self.advance();

					self.gen_token(TokenId::Tab)
				},
				'\n' => {
					self.advance();

					self.gen_token(TokenId::Line)
				},
				_ => self.gen_error(ErrorId::InvalidChar, 0)
			}
		}
	}
}