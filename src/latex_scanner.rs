mod latex;

pub struct Text<'a>(&'a str);
pub struct Optional {

}
pub struct Argument()
pub struct Command<'a> {
    command: &'a str,
    arguments:
}
pub enum Token<'a> {
    Text(Text<'a>),
    Optional(Optional),
}