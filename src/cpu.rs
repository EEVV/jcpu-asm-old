#[derive(Debug, Clone)]
pub enum Opcode {
	Mov = 0,
	Not = 1,
	Or = 2,
	Nor = 3,
	And = 4,
	Nand = 5,
	Xor = 6,
	Xnor = 7,
	Neg = 8,
	Add = 9,
	Sub = 10,
	Lt = 11,
	Nlt = 12,
	Slt = 13,
	Nslt = 14,
	Sl = 15,
	Sr = 16,
	Ssl = 17,
	Ssr = 18,
	Rep = 19,
	Mul = 20,
	Div = 21,
	Sto8 = 22,
	Sto16 = 23,
	Sto32 = 24,
	Lod8 = 25,
	Lod16 = 26,
	Lod32 = 27,
	Done = 28
}

#[derive(Debug)]
pub struct Inst {
	pub opcode: Opcode,
	pub ce: bool,
	pub ci: bool,
	pub cond: u8,
	pub dest1: u8,
	pub dest0: u8,
	pub src1: u8,
	pub src0: u8,
	pub w1: bool,
	pub w0: bool,
	pub i1: bool,
	pub i0: bool,
	pub imm1: u32,
	pub imm0: u32
}

impl Inst {
	pub fn new() -> Inst {
		Inst {
			opcode: Opcode::Mov,
			ce: false,
			ci: false,
			cond: 0,
			dest1: 0,
			dest0: 0,
			src1: 0,
			src0: 0,
			w1: false,
			w0: false,
			i1: false,
			i0: false,
			imm1: 0,
			imm0: 0
		}
	}

	pub fn gen(self) -> Vec<u32> {
		let mut inst: u32 = self.opcode as u32;
		inst <<= 1;
		inst |= self.ce as u32;
		inst <<= 1;
		inst |= self.ci as u32;
		inst <<= 4;
		inst |= (self.cond & 0b1111) as u32;
		inst <<= 4;
		inst |= (self.dest1 & 0b1111) as u32;
		inst <<= 4;
		inst |= (self.dest0 & 0b1111) as u32;
		inst <<= 4;
		inst |= (self.src1 & 0b1111) as u32;
		inst <<= 4;
		inst |= (self.src0 & 0b1111) as u32;
		inst <<= 1;
		inst |= self.w1 as u32;
		inst <<= 1;
		inst |= self.w0 as u32;
		inst <<= 1;
		inst |= self.i1 as u32;
		inst <<= 1;
		inst |= self.i0 as u32;

		let mut insts = vec![inst];

		if self.i0 {
			insts.push(self.imm0)
		}

		if self.i1 {
			insts.push(self.imm1)
		}

		insts
	}
}
