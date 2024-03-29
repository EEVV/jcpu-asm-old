use loc::Loc;
use token::Token;


#[derive(Debug, Clone)]
pub enum ErrorId {
	// Errors generated by lexer
	InvalidReg,
	InvalidChar,

	// Errors generated by parser
	ExpectedProgram,
	ExpectedLine,
	ExpectedAtom,
	ExpectedParen,
	ExpectedSquare,

	InvalidNode,
	InvalidInstruction
}

#[derive(Debug, Clone)]
pub struct Error {
	pub id: ErrorId,
	pub loc: Loc
}


pub fn gen_error<T>(error_id: ErrorId, token: Token) -> Result<T, Error> {
	Err(Error {
		id: error_id,
		loc: token.loc.clone(),
	})
}

/*
pub fn print_error(error: &Error) {

}
*/