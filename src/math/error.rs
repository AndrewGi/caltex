pub enum Error {
    UndefinedVariable(String),
    UndefinedFunction(String),
    BadArguments,
    DivideByZero,
    NaN,
}
