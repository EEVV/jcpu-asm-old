#[derive(Debug)]
pub enum Node {
	Num(isize),
	Iden(String),
	Reg(isize),
	Label(String),

	Not(Box<Node>),
	Neg(Box<Node>),
	Rep(Box<Node>),

	Or(Box<Node>, Box<Node>),
	And(Box<Node>, Box<Node>),
	Xor(Box<Node>, Box<Node>),
	Add(Box<Node>, Box<Node>),
	Sub(Box<Node>, Box<Node>),
	Mul(Box<Node>, Box<Node>),
	Div(Box<Node>, Box<Node>),

	Eql(Box<Node>, Box<Node>),
	Gt(Box<Node>, Box<Node>),
	Lt(Box<Node>, Box<Node>),

	Opers(Vec<Node>),

	To(Box<Node>, Box<Node>),
	Cond(Box<Node>, Box<Node>, Box<Node>)
}
