use token::{TokenId, Token};
use lexer::Lexer;
use error::{ErrorId, Error};
use node::Node;


struct Parser {
	lexer: Lexer,
	token_result: Result<Token, Error>
}

impl Parser {
	fn advance(&mut self) {
		self.token_result = self.lexer.token();
	}

	fn gen_error<T>(&mut self, error_id: ErrorId, token: Token) -> Result<T, Error> {
		Err(Error {
			id: error_id,
			loc: token.loc.clone(),
		})
	}

	fn parse_atom(&mut self) -> Result<Node, Error> {
		match self.token_result.clone() {
			Err(err) => Err(err),
			Ok(token) => match token.id {
				TokenId::Num(num) => {
					let node = Node::Num(num);

					self.advance();

					Ok(node)
				},
				TokenId::Iden(iden) => {
					let node = Node::Iden(iden.clone());

					self.advance();

					Ok(node)
				},
				TokenId::Reg(reg) => {
					let node = Node::Reg(reg);

					self.advance();

					Ok(node)
				},
				TokenId::Not => {
					self.advance();

					let node_result = self.parse_atom();

					match node_result {
						Err(_) => node_result,
						Ok(node) => Ok(Node::Not(Box::new(node)))
					}
				},
				TokenId::Sub => {
					self.advance();

					let node_result = self.parse_atom();

					match node_result {
						Err(_) => node_result,
						Ok(node) => Ok(Node::Neg(Box::new(node)))
					}
				},
				TokenId::Div => {
					self.advance();

					let node_result = self.parse_atom();

					match node_result {
						Err(_) => node_result,
						Ok(node) => Ok(Node::Rep(Box::new(node)))
					}
				},
				TokenId::ParenL => {
					self.advance();

					let node_result = self.parse_opers();

					match node_result {
						Err(_) => node_result,
						Ok(_) => {
							match self.token_result.clone() {
								Err(err) => Err(err),
								Ok(token) => match token.id {
									TokenId::ParenR => {
										self.advance();

										node_result
									},
									_ => self.gen_error(ErrorId::ExpectedParen, token.clone())
								}
							}
						}
					}
				},
				_ => self.gen_error(ErrorId::ExpectedAtom, token.clone())
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
								Ok(right) => node = Node::Gt(Box::new(node), Box::new(right))
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

	fn parse_opers(&mut self) -> Result<Node, Error> {
		let node_result = self.parse_oper();

		match node_result {
			Err(_) => node_result,
			Ok(node) => match self.token_result.clone() {
				Err(err) => Err(err),
				Ok(token) => match token.id {
					TokenId::Comma => {
						self.advance();

						let mut nodes = vec![node];
						let node_result = self.parse_oper();

						match node_result {
							Err(_) => node_result,
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
													Err(_) => return node_result,
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
											Ok(cond) => Ok(Node::Cond(Box::new(left), Box::new(right), Box::new(cond)))
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

	fn parse(&mut self) -> Result<Vec<Node>, Error> {
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
								_ => return self.gen_error(ErrorId::ExpectedLine, token)
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
								_ => return self.gen_error(ErrorId::ExpectedLine, token)
							}
						}

						self.advance();
					},
					TokenId::Line => {
						self.advance();
					},
					TokenId::Eof => break,
					_ => return self.gen_error(ErrorId::ExpectedProgram, token.clone())
				}
			}
		}

		Ok(nodes)
	}
}

pub fn parse(source: String) -> Result<Vec<Node>, Error> {
	let mut lexer = Lexer::new(source);
	let token_result = lexer.token();

	let mut parser = Parser {
		lexer: lexer,
		token_result: token_result
	};

	parser.parse()
}