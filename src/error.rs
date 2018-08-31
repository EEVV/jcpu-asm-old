use loc::Loc;


#[derive(Debug, Clone)]
pub enum ErrorId {
	// Errors generated by lexer
	InvalidReg,
	InvalidChar,

	// Errors generated by parser
	ExpectedProgram,
	ExpectedLine,
	ExpectedAtom,
	ExpectedParen
}

#[derive(Debug, Clone)]
pub struct Error {
	pub id: ErrorId,
	pub loc: Loc
}

/*
pub fn print_error(error: &Error) {

}
*/