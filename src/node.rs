use cpu::{Opcode, Inst};
use error::{ErrorId, Error, gen_error};


#[derive(Debug)]
pub struct Program {
	pub nodes: Vec<Node>
}

impl Program {
	pub fn gen(self) -> Result<Vec<u32>, ErrorId> {
		let mut raw_insts = Vec::new();
		let mut address = 0;

		for node in self.nodes {
			let raw_insts0_result = node.gen(&mut address);
			match raw_insts0_result {
				Err(_) => return raw_insts0_result,
				Ok(mut raw_insts0) => raw_insts.append(&mut raw_insts0)
			}
		}

		Ok(raw_insts)
	}
}

#[derive(Debug)]
pub enum Node {
	Num(isize),
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
	Mul(Box<Node>, Box<Node>),
	Div(Box<Node>, Box<Node>),

	Eql(Box<Node>, Box<Node>),
	Lt(Box<Node>, Box<Node>),

	Opers(Vec<Node>),

	To(Box<Node>, Box<Node>),
	Cond(Box<Node>, Box<Node>),
}


impl Node {
	fn gen_src0(&self, mut inst: Inst) -> Result<Vec<u32>, ErrorId> {
		match self {
			Node::Num(num) => {
				inst.i0 = true;
				inst.imm0 = *num as u32;

				Ok(inst.gen())
			},
			Node::Reg(reg) => {
				inst.src0 = *reg;

				Ok(inst.gen())
			}
			_ => Err(ErrorId::InvalidInstruction)
		}
	}

	fn gen_both(&self, left: &Node, mut inst: Inst) -> Result<Vec<u32>, ErrorId> {
		match self {
			Node::Num(num) => {
				inst.i1 = true;
				inst.imm1 = *num as u32;

				left.gen_src0(inst)
			},
			Node::Reg(reg) => {
				inst.src1 = *reg;

				left.gen_src0(inst)
			}
			_ => Err(ErrorId::InvalidInstruction)
		}
	}

	fn gen_inst(&self, mut inst: Inst) -> Result<Vec<u32>, ErrorId> {
		match self {
			Node::To(box left, box right) => match left {
				Node::Reg(reg) => {
					inst.dest0 = *reg;
					inst.w0 = true;

					match right {
						Node::Not(box node) => {
							inst.opcode = Opcode::Not;

							node.gen_src0(inst)
						},
						Node::Neg(box node) => {
							inst.opcode = Opcode::Neg;

							node.gen_src0(inst)
						},
						Node::Rep(box node) => {
							inst.opcode = Opcode::Rep;

							node.gen_src0(inst)
						},
						Node::Mem(box node) => {
							inst.opcode = Opcode::Lod32;

							node.gen_src0(inst)
						},
						Node::Or(box left0, box right0) => {
							inst.opcode = Opcode::Or;

							right0.gen_both(left0, inst)
						},
						Node::And(box left0, box right0) => {
							inst.opcode = Opcode::And;

							right0.gen_both(left0, inst)
						},
						Node::Xor(box left0, box right0) => {
							inst.opcode = Opcode::Xor;

							right0.gen_both(left0, inst)
						},
						Node::Add(box left0, box right0) => {
							inst.opcode = Opcode::Add;

							right0.gen_both(left0, inst)
						},
						Node::Sub(box left0, box right0) => {
							inst.opcode = Opcode::Sub;

							right0.gen_both(left0, inst)
						},
						Node::Mul(box left0, box right0) => {
							inst.opcode = Opcode::Mul;

							right0.gen_both(left0, inst)
						},
						Node::Div(box left0, box right0) => {
							inst.opcode = Opcode::Div;

							right0.gen_both(left0, inst)
						},
						/*
						TODO:

						Node::Eql(box left0, box right0) => {
							inst.opcode = Opcode::Eql;

							right0.gen_both(left0, inst)
						},
						*/
						Node::Lt(box left0, box right0) => {
							inst.opcode = Opcode::Lt;

							right0.gen_both(left0, inst)
						},
						_ => right.gen_src0(inst)
					}
				},
				Node::Mem(box mem) => {
					inst.opcode = Opcode::Sto32;

					match mem {
						Node::Num(num) => {
							inst.i0 = true;
							inst.imm0 = *num as u32;
						},
						Node::Reg(reg) => {
							inst.dest0 = *reg;
							inst.w0 = true;
						},
						_ => ()
					}

					right.gen_src0(inst)
				},
				_ => Err(ErrorId::InvalidInstruction)
			},
			_ => Err(ErrorId::InvalidInstruction)
		}
	}

	fn gen_lt(&self, mut inst: Inst, cond_to: Node) -> Result<Vec<u32>, ErrorId> {
		match self {
			Node::Lt(box lt_left, box lt_right) => match lt_left {
				Node::Num(0) => match lt_right {
					Node::Reg(reg) => {
						inst.cond = *reg;

						cond_to.gen_inst(inst)
					},
					_ => Err(ErrorId::InvalidInstruction)
				},
				_ => Err(ErrorId::InvalidInstruction)
			},
			_ => Err(ErrorId::InvalidInstruction)
		}
	}

	fn gen(self, address: &mut usize) -> Result<Vec<u32>, ErrorId> {
		match self {
			Node::To(_, _) => self.gen_inst(Inst::new()),
			Node::Cond(box cond_to, box cond) => {
				let mut inst = Inst::new();
				inst.ce = true;

				match cond {
					Node::Not(node) => {
						inst.ci = true;

						node.gen_lt(inst, cond_to)
					},
					Node::Lt(_, _) => cond.gen_lt(inst, cond_to),
					_ => Err(ErrorId::InvalidInstruction)
				}
			},
			_ => Err(ErrorId::InvalidNode)
		}
	}
}
