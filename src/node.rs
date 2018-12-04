use std::collections::HashMap;
use std::mem;

use cpu::{Opcode, Inst};
use error::{ErrorId, Error, gen_error};


#[derive(Debug)]
pub struct Program {
	pub nodes: Vec<Node>,
	// binary output
	pub binary: Vec<u32>,
	// list of labels
	pub labels: HashMap<String, i32>,
	// labels to be filled
	pub queue: HashMap<String, Vec<usize>>,
	// current address
	pub addr: usize
}

impl Program {
	pub fn gen(mut self) -> Option<Vec<u32>> {
		let mut nodes = vec![];
		mem::swap(&mut nodes, &mut self.nodes);

		for node in nodes {
			if node.gen(&mut self) == false {
				return None
			}
		}

		Some(self.binary)
	}

	fn get_iden(&mut self, iden: String, offset: usize) -> u32 {
		match self.labels.get(&iden) {
			None => {
				self.queue.entry(iden).or_insert(vec![]).push(self.addr + offset);
				0
			},
			Some(num) => *num as u32
		}
	}
}

#[derive(Debug, Clone)]
pub enum Node {
	Num(i32),
	Iden(String),
	Reg(u8),
	Label(String),
	Empty,

	Not(Box<Node>),
	Neg(Box<Node>),
	Rep(Box<Node>),
	Mem(Box<Node>),

	Or(Box<Node>, Box<Node>),
	And(Box<Node>, Box<Node>),
	Xor(Box<Node>, Box<Node>),
	Add(Box<Node>, Box<Node>),
	Sub(Box<Node>, Box<Node>),
	Sl(Box<Node>, Box<Node>),
	Sr(Box<Node>, Box<Node>),
	Mul(Box<Node>, Box<Node>),
	Div(Box<Node>, Box<Node>),

	Eql(Box<Node>, Box<Node>),
	Lt(Box<Node>, Box<Node>),

	Mem8(Box<Node>),
	Mem16(Box<Node>),
	Mem32(Box<Node>),

	Opers(Vec<Node>),

	To(Box<Node>, Box<Node>),
	Cond(Box<Node>, Box<Node>),
}

impl Node {
	fn gen_src0(self, program: &mut Program, inst: &mut Inst) -> bool {
		match self {
			Node::Num(num) => {
				inst.i0 = true;
				inst.imm0 = num as u32;
			},
			Node::Reg(reg) => {
				inst.src0 = reg;
			},
			Node::Iden(iden) => {
				inst.i0 = true;
				inst.imm0 = program.get_iden(iden, 1);
			},
			_ => return false
		}

		true
	}

	fn gen_src1(self, program: &mut Program, inst: &mut Inst) -> bool {
		match self {
			Node::Num(num) => {
				inst.i1 = true;
				inst.imm1 = num as u32;
			},
			Node::Reg(reg) => {
				inst.src1 = reg;
			},
			Node::Iden(iden) => {
				inst.i1 = true;
				inst.imm1 = program.get_iden(iden, 1);
			},
			_ => return false
		}

		true
	}

	fn gen_uncond(self, program: &mut Program, inst: &mut Inst) -> bool {
		match self {
			// gen instructions
			Node::To(box left, box right) => {
				match left {
					Node::Reg(reg) => {
						inst.w0 = true;
						inst.dest0 = reg;

						// TODO:
						// elevate right side to make it easier
						// Node(x) -> x
						// then after that handle src0, src1, dest0, dest1
						match right {
							Node::Num(num) => {
								inst.i0 = true;
								inst.imm0 = num as u32;
							},
							Node::Reg(reg) => {
								inst.src0 = reg;
							},
							Node::Iden(iden) => {
								inst.i0 = true;
								inst.imm0 = program.get_iden(iden, 1);
							},
							Node::Not(box node) => match node {
								// 1 op
								Node::Num(num) => {
									inst.opcode = Opcode::Not;
									inst.i0 = true;
									inst.imm0 = num as u32;
								},
								Node::Reg(reg) => {
									inst.opcode = Opcode::Not;
									inst.src0 = reg;
								},
								Node::Iden(iden) => {
									inst.opcode = Opcode::Not;
									inst.i0 = true;
									inst.imm0 = program.get_iden(iden, 1);
								},
								// 2 op, optimization
								Node::Or(box left, box right) => {
									inst.opcode = Opcode::Nor;

									if !left.gen_src0(program, inst) {
										return false
									}

									if !right.gen_src1(program, inst) {
										return false
									}
								},
								Node::And(box left, box right) => {
									inst.opcode = Opcode::Nand;

									if !left.gen_src0(program, inst) {
										return false
									}

									if !right.gen_src1(program, inst) {
										return false
									}
								},
								Node::Xor(box left, box right) => {
									inst.opcode = Opcode::Xnor;

									if !left.gen_src0(program, inst) {
										return false
									}

									if !right.gen_src1(program, inst) {
										return false
									}
								},
								_ => return false
							},
							Node::Neg(box node) => {
								inst.opcode = Opcode::Neg;

								if !node.gen_src0(program, inst) {
									return false
								}
							},
							Node::Rep(box node) => {
								inst.opcode = Opcode::Rep;

								if !node.gen_src0(program, inst) {
									return false
								}
							},
							Node::Or(box left, box right) => {
								inst.opcode = Opcode::Or;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::And(box left, box right) => {
								inst.opcode = Opcode::And;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Xor(box left, box right) => {
								inst.opcode = Opcode::Xor;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Add(box left, box right) => {
								inst.opcode = Opcode::Add;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Sub(box left, box right) => {
								inst.opcode = Opcode::Sub;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Lt(box left, box right) => {
								inst.opcode = Opcode::Lt;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Sl(box left, box right) => {
								inst.opcode = Opcode::Sl;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Sr(box left, box right) => {
								inst.opcode = Opcode::Sr;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Mul(box left, box right) => {
								inst.opcode = Opcode::Mul;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Div(box left, box right) => {
								inst.opcode = Opcode::Div;

								if !left.gen_src0(program, inst) {
									return false
								}

								if !right.gen_src1(program, inst) {
									return false
								}
							},
							Node::Mem8(box node) => {
								inst.opcode = Opcode::Lod8;

								match node {
									Node::Num(num) => {
										inst.i0 = true;
										inst.imm0 = num as u32;
									},
									Node::Reg(reg) => {
										inst.src0 = reg;
									},
									_ => return false
								}
							},
							Node::Mem16(box node) => {
								inst.opcode = Opcode::Lod16;

								match node {
									Node::Num(num) => {
										inst.i0 = true;
										inst.imm0 = num as u32;
									},
									Node::Reg(reg) => {
										inst.src0 = reg;
									},
									_ => return false
								}
							},
							Node::Mem32(box node) => {
								inst.opcode = Opcode::Lod32;

								match node {
									Node::Num(num) => {
										inst.i0 = true;
										inst.imm0 = num as u32;
									},
									Node::Reg(reg) => {
										inst.src0 = reg;
									},
									_ => return false
								}
							},
							_ => return false
						}
					},
					Node::Mem8(box node) => {
						inst.opcode = Opcode::Sto8;

						match node {
							Node::Num(num) => {
								inst.i0 = true;
								inst.imm0 = num as u32;
							},
							Node::Reg(reg) => {
								inst.src0 = reg;
							},
							_ => return false
						}

						match right {
							Node::Num(num) => {
								inst.i1 = true;
								inst.imm1 = num as u32;
							},
							Node::Reg(reg) => {
								inst.src1 = reg;
							},
							_ => return false
						}
					},
					Node::Mem16(box node) => {
						inst.opcode = Opcode::Sto16;

						match node {
							Node::Num(num) => {
								inst.i0 = true;
								inst.imm0 = num as u32;
							},
							Node::Reg(reg) => {
								inst.src0 = reg;
							},
							_ => return false
						}

						match right {
							Node::Num(num) => {
								inst.i1 = true;
								inst.imm1 = num as u32;
							},
							Node::Reg(reg) => {
								inst.src1 = reg;
							},
							_ => return false
						}
					},
					Node::Mem32(box node) => {
						inst.opcode = Opcode::Sto32;

						match node {
							Node::Num(num) => {
								inst.i0 = true;
								inst.imm0 = num as u32;
							},
							Node::Reg(reg) => {
								inst.src0 = reg;
							},
							_ => return false
						}

						match right {
							Node::Num(num) => {
								inst.i1 = true;
								inst.imm1 = num as u32;
							},
							Node::Reg(reg) => {
								inst.src1 = reg;
							},
							_ => return false
						}
					},
					_ => return false
				}
			},
			_ => return false
		}

		true
	}

	fn gen(self, program: &mut Program) -> bool {
		let mut inst = Inst::new();

		match self {
			// gen direct data
			Node::Num(num) => {
				program.binary.push(num as u32);
				program.addr += 1;
			},
			Node::Iden(iden) => {
				let iden_u32 = program.get_iden(iden, 0);
				program.binary.push(iden_u32);

				program.addr += 1;
			},
			Node::Label(label) => {
				match program.queue.remove(&label) {
					None => (),
					Some(addrs) => {
						for addr in addrs {
							program.binary[addr] = program.addr as u32
						}
					}
				}

				program.labels.insert(label, program.addr as i32);
			},
			// negative numbers
			Node::Neg(box node) => match node {
				Node::Num(num) => {
					program.binary.push((-num) as u32);
					program.addr += 1;
				},
				_ => return false
			},

			Node::Cond(box node, box cond) => {
				match cond {
					Node::Not(box node) => match node {
						Node::Eql(box left, box right) => match left {
							Node::Num(0) => match right {
								Node::Reg(reg) => {
									inst.ce = true;
									inst.cond = reg;
								},
								_ => return false
							},
							Node::Reg(reg) => match right {
								Node::Num(0) => {
									inst.ce = true;
									inst.cond = reg;
								},
								_ => return false
							}
							_ => return false
						},
						_ => return false
					},
					_ => return false
				}

				node.gen_uncond(program, &mut inst);
			},
			_ => {
				if !self.gen_uncond(program, &mut inst) {
					return false
				}
			}
		}

		let mut inst_raw = inst.gen();
		program.addr += inst_raw.len();
		program.binary.append(&mut inst_raw);

		true
	}
}
