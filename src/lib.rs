#![feature(conservative_impl_trait)]
extern crate combine;

//pub mod parse;
pub mod ast;

use combine::char::{digit, space, spaces, string};
pub use combine::Parser;
use combine::{many, satisfy, sep_end_by, skip_many, token, Stream, skip_many1};
use combine::range::take_while;
use combine::primitives::RangeStream;
pub fn field<'a, I>() -> impl Parser<Input = I, Output = ast::Field<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    (
        spaces(),
        take_while(is_ident),
        spaces(),
        token(':'),
        spaces(),
        take_while(is_ident),
    ).map(|(_, name, _, _, _, ty_name)| {
        ast::Field {
            ident: ast::Ident(name),
            ty: ast::Ident(ty_name),
        }
    })
}

pub fn digits<I>() -> impl Parser<Input = I, Output = Vec<char>>
where
    I: Stream<Item = char>,
{
    sep_end_by(digit(), token(',').skip(spaces()))
}

pub fn fields<'a, I>() -> impl Parser<Input = I, Output = Vec<ast::Field<'a>>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    sep_end_by(field().skip(spaces()), token(',').skip(whitespace()))
}

pub fn is_ident(c: char) -> bool {
    c.is_alphanumeric()
}
pub fn data<'a, I>() -> impl Parser<Input = I, Output = ast::Data<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    (
        spaces(),
        string("data").skip(spaces()),
        take_while(is_ident).skip(spaces()),
        token('{').skip(spaces()),
        fields().skip(spaces()),
        token('}').skip(spaces()),
    ).map(|(_, _, name, _, fields, _)| {
        ast::Data {
            ident: ast::Ident(name),
            fields: fields,
        }
    })
}

pub fn object<'a, I>() -> impl Parser<Input = I, Output = ast::Object<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    (
        spaces(),
        string("object"),
        space(),
        take_while(|c: char| !c.is_whitespace() && c != ';'),
        spaces(),
        token(';'),
        whitespace(),
    ).map(|(_, _, _, ident, _, _, _)| {
        ast::Object::new(ast::Ident(ident))
    })
}

fn whitespace<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
{
    let comment = (token('/'), token('/'), skip_many(satisfy(|c| c != '\n'))).map(|_| ());
    skip_many(skip_many1(space()).or(comment))
}



pub fn entry<'a, I>() -> impl Parser<Input = I, Output = Vec<ast::Ast<'a>>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    let ast_object = object().map(|o| ast::Ast::Object(o));
    let ast_data = data().map(|e| ast::Ast::Data(e));
    many(ast_object.or(ast_data).skip(whitespace()))
}
