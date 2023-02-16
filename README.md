# parseme
Parseme is a simple parser combinator framework which takes advantage of Rust's type system.

## Examples
A basic identifier parser using Unicode XID via `parseme::xid` (requires the `xid` feature, enabled by default).
```rust
use parseme::Source;

fn parse_iden<'a>(input: &mut Source<'a>) -> Result<&'a str, ()> {
    let start_pos = input.pos();

    parseme::iter::next_if(input, parseme::xid::is_start).ok_or(())?;
    parseme::iter::advance_while(input, parseme::xid::is_continue);

    Ok(&input.src()[start_pos..input.pos()])
}

fn main() {
    let ident1 = "hello";
    let ident2 = "hello123";

    let bad1 = "123";
    let bad2 = "_ident";

    assert_eq!(parse_iden(&mut Source::new(ident1)), Ok("hello"));
    assert_eq!(parse_iden(&mut Source::new(ident2)), Ok("hello123"));
    assert_eq!(parse_iden(&mut Source::new(bad1)), Err(()));
    assert_eq!(parse_iden(&mut Source::new(bad2)), Err(()));
}
```