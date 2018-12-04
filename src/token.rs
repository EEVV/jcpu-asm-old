use loc::Loc;


#[derive(Debug, Clone, PartialEq)]
pub enum TokenId {
	Iden(String),
	Num(isize),
	Reg(u8),
	Empty,

	Mem8,
	Mem16,
	Mem32,

	Not,
	Or,
	And,
	Xor,
	Add,
	Sub,
	Sl,
	Sr,
	Mul,
	Div,
	Eql,
	Lt,
	Gt,

	Comma,

	To,

	If,

	ParenL,
	ParenR,
	SquareL,
	SquareR,

	Tab,
	Line,

	Eof
}

#[derive(Debug, Clone)]
pub struct Token {
	pub id: TokenId,
	pub loc: Loc
}