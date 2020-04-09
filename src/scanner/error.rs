pub struct EOLError(pub ());
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidState,
    EOL,
    UnexpectedCharacter(char),
    ExpectedCharacter(char),
    ExpectedCharacters(&'static str),
}

impl From<EOLError> for Error {
    fn from(_: EOLError) -> Self {
        Error::EOL
    }
}
