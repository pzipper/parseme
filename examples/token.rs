use parseme::{parser::Group, stream::Stream, NoMatchError, Parser, Source};

#[derive(Debug, PartialEq)]
pub enum Token {
    Whitespace,
    Ident(String),
}

fn parse_whitespace(input: &mut Source) -> Result<Token, NoMatchError> {
    parseme::iter::next_if(input, char::is_whitespace).ok_or(NoMatchError)?;

    Ok(Token::Whitespace)
}

fn parse_iden<'a>(input: &mut Source<'a>) -> Result<Token, NoMatchError> {
    let start_pos = input.pos();

    parseme::iter::next_if(input, parseme::xid::is_start).ok_or(NoMatchError)?;
    parseme::iter::advance_while(input, parseme::xid::is_continue);

    Ok(Token::Ident(
        input.src()[start_pos..input.pos()].to_string(),
    ))
}

fn main() {
    let mut source = Source::new("hello world");
    let mut group = Group::new().add(parse_whitespace).add(parse_iden);
    let mut parser = group.stream(&mut source);

    assert_eq!(parser.next(), Ok(Token::Ident("hello".to_string())));
    assert_eq!(parser.next(), Ok(Token::Whitespace));
    assert_eq!(parser.next(), Ok(Token::Ident("world".to_string())));
}
