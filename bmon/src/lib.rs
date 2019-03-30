#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Link(&'a str), // Will be hotlinked if rendered as HTML
    Sequence(&'a [Value<'a>]),
    Object(&'a [(Value<'a>, Value<'a>)]), // Easier to make literals of
    Number(i64),
    Boolean(bool),
    Null,
}
