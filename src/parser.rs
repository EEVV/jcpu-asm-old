use std::collections::HashMap;

use token::{TokenId, Token};
use lexer::Lexer;
use error::{ErrorId, Error, gen_error};
use node::{Program, Node};


struct Parser {
	lexer: Lexer,
	token_result: Result<Token, Error>
}

impl Parser {
	fn advance(&mut self) {
		self.token_result = self.lexer.token();
	}

	fn parse_paren(&mut self) -> Result<Node, Error> {
		match self.token_result.clone() {
			Err(err) => Err(err),
			Ok(token) => match token.id {
				TokenId::ParenL => {
					self.advance();

					let node_result = self.parse_opers();

					match node_result {
						Err(_) => node_result,
						Ok(_) => match self.token_result.clone() {
							Err(err) => Err(err),
							Ok(token) => match token.id {
								TokenId::ParenR => {
									self.advance();

									node_result
								},
								_ => gen_error(ErrorId::ExpectedParen, token.clone())
							}
						}
					}
				},
				_ => gen_error(ErrorId::ExpectedAtom, token.clone())
			}
		}
	}

	fn parse_atom(&mut self) -> Result<Node, Error> {
		match self.token_result.clone() {
			Err(err) => Err(err),
			Ok(token) => match token.id {
				// parse a number
				TokenId::Num(num) => {
					let node = Node::Num(num as i32);

					self.advance();

					Ok(node)
				},
				// parse identifier
				TokenId::Iden(iden) => {
					let node = Node::Iden(iden.clone());

					self.advance();

					Ok(node)
				},
				// parse register
				TokenId::Reg(reg) => {
					let node = Node::Reg(reg);

					self.advance();

					Ok(node)
				},
				// parse `_`
				TokenId::Empty => {
					self.advance();

					Ok(Node::Empty)
				},
				// parse !x
				TokenId::Not => {
					self.advance();

					let node_result = self.parse_atom();

					match node_result {
						Err(_) => node_result,
						Ok(node) => Ok(Node::Not(Box::new(node)))
					}
				},
				// parse -x
				TokenId::Sub => {
					self.advance();

					let node_result = self.parse_atom();

					match node_result {
						Err(_) => node_result,
						Ok(node) => Ok(Node::Neg(Box::new(node)))
					}
				},
				// parse /x
				TokenId::Div => {
					self.advance();

					let node_result = self.parse_atom();

					match node_result {
						Err(_) => node_result,
						Ok(node) => Ok(Node::Rep(Box::new(node)))
					}
				},
				TokenId::Mem8 => {
					self.advance();

					match self.parse_paren() {
						Err(err) => Err(err),
						Ok(node) => Ok(Node::Mem8(Box::new(node)))
					}
				},
				TokenId::Mem16 => {
					self.advance();

					match self.parse_paren() {
						Err(err) => Err(err),
						Ok(node) => Ok(Node::Mem16(Box::new(node)))
					}
				},
				TokenId::Mem32 => {
					self.advance();

					match self.parse_paren() {
						Err(err) => Err(err),
						Ok(node) => Ok(Node::Mem32(Box::new(node)))
					}
				},
				_ => self.parse_paren()
			}
		}
	}

	fn parse_oper(&mut self) -> Result<Node, Error> {
		let node_result = self.parse_atom();

		match node_result {
			Err(_) => node_result,
			Ok(mut node) => loop {
				match self.token_result.clone() {
					Err(err) => return Err(err),
					Ok(token) => match token.id {
						TokenId::Or => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Or(Box::new(node), Box::new(right))
							}
						},
						TokenId::And => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::And(Box::new(node), Box::new(right))
							}
						},
						TokenId::Xor => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Xor(Box::new(node), Box::new(right))
							}
						},
						TokenId::Add => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Add(Box::new(node), Box::new(right))
							}
						},
						TokenId::Sub => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Sub(Box::new(node), Box::new(right))
							}
						},
						TokenId::Sl => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Sl(Box::new(node), Box::new(right))
							}
						},
						TokenId::Sr => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Sr(Box::new(node), Box::new(right))
							}
						},
						TokenId::Mul => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Mul(Box::new(node), Box::new(right))
							}
						},
						TokenId::Div => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Div(Box::new(node), Box::new(right))
							}
						},
						TokenId::Eql => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Eql(Box::new(node), Box::new(right))
							}
						},
						TokenId::Gt => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Lt(Box::new(right), Box::new(node))
							}
						},
						TokenId::Lt => {
							self.advance();

							let right_result = self.parse_atom();
							match right_result {
								Err(_) => return right_result,
								Ok(right) => node = Node::Lt(Box::new(node), Box::new(right))
							}
						},
						_ => return Ok(node)
					}
				}
			}
		}
	}

	// parse operands
	// x, y, z, ..., w
	fn parse_opers(&mut self) -> Result<Node, Error> {
		let node_result = self.parse_oper();

		match node_result {
			Err(err) => Err(err),
			Ok(node) => match self.token_result.clone() {
				Err(err) => Err(err),
				Ok(token) => match token.id {
					TokenId::Comma => {
						self.advance();

						let mut nodes = vec![node];

						let node_result = self.parse_oper();

						match node_result {
							Err(err) => Err(err),
							Ok(node) => {
								nodes.push(node);

								loop {
									match self.token_result.clone() {
										Err(err) => return Err(err),
										Ok(token) => match token.id {
											TokenId::Comma => {
												self.advance();

												let node_result = self.parse_oper();

												match node_result {
													Err(err) => return Err(err),
													Ok(node) => nodes.push(node)
												}
											},
											_ => return Ok(Node::Opers(nodes))
										}
									}
								}
							}
						}
					},
					_ => return Ok(node)
				}
			}
		}
	}

	fn parse_to(&mut self) -> Result<Node, Error> {
		let left_result = self.parse_opers();

		match left_result {
			Err(_) => left_result,
			Ok(left) => match self.token_result.clone() {
				Err(err) => Err(err),
				Ok(token) => match token.id {
					TokenId::To => {
						self.advance();

						let right_result = self.parse_opers();
						match right_result {
							Err(_) => right_result,
							Ok(right) => match self.token_result.clone() {
								Err(err) => Err(err),
								Ok(token) => match token.id {
									TokenId::If => {
										self.advance();

										let cond_result = self.parse_oper();
										match cond_result {
											Err(err) => Err(err),
											Ok(cond) => Ok(Node::Cond(Box::new(Node::To(Box::new(left), Box::new(right))), Box::new(cond)))
										}
									},
									_ => Ok(Node::To(Box::new(left), Box::new(right)))
								}
							}
						}
					},
					_ => Ok(left)
				}
			}
		}
	}

	fn parse(&mut self) -> Result<Program, Error> {
		let mut nodes = Vec::new();

		loop {
			match self.token_result.clone() {
				Err(err) => return Err(err),
				Ok(token) => match token.id {
					TokenId::Tab => {
						self.advance();

						let node_or_err = self.parse_to();

						match node_or_err {
							Err(err) => return Err(err),
							Ok(node) => {
								nodes.push(node);
							}
						}

						match self.token_result.clone() {
							Err(err) => return Err(err.clone()),
							Ok(token) => match token.id {
								TokenId::Line => self.advance(),
								TokenId::Eof => break,
								_ => return gen_error(ErrorId::ExpectedLine, token)
							}
						}
					},
					TokenId::Iden(iden) => {
						// Parse label
						nodes.push(Node::Label(iden.clone()));

						self.advance();

						match self.token_result.clone() {
							Err(err) => return Err(err.clone()),
							Ok(token) => match token.id {
								TokenId::Line | TokenId::Eof => (),
								_ => return gen_error(ErrorId::ExpectedLine, token)
							}
						}

						self.advance();
					},
					TokenId::Line => {
						self.advance();
					},
					TokenId::Eof => break,
					_ => return gen_error(ErrorId::ExpectedProgram, token.clone())
				}
			}
		}

		Ok(Program {
			nodes: nodes,
			binary: Vec::new(),
			labels: HashMap::new(),
			queue: HashMap::new(),
			addr: 0
		})
	}
}

pub fn parse(source: String) -> Result<Program, Error> {
	let mut lexer = Lexer::new(source);
	let token_result = lexer.token();

	let mut parser = Parser {
		lexer: lexer,
		token_result: token_result
	};

	parser.parse()
}