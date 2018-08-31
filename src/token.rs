use loc::Loc;


#[derive(Debug, Clone, PartialEq)]
pub enum TokenId {
	Iden(String),
	Num(isize),
	Reg(isize),

	Not,
	Or,
	And,
	Xor,
	Add,
	Sub,
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