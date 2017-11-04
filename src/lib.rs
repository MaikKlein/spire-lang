#![feature(conservative_impl_trait)]
#[macro_use]
extern crate combine;

//pub mod parse;
pub mod ast;

use combine::char::{char, digit, space, spaces, string};
pub use combine::Parser;
use combine::{choice, many, optional, satisfy, sep_by, sep_end_by, skip_many, token, Stream,
              many1, skip_many1};
use combine::range::take_while;
use combine::StreamOnce;
use combine::primitives::{Range, RangeStream};
use combine::ParseError;
use std::collections::HashMap;
pub fn field<'a, I>() -> impl Parser<Input = I, Output = ast::Field<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    (
        take_while(|c: char| !c.is_whitespace() && c != ':'),
        spaces(),
        token(':'),
        spaces(),
        take_while(|c: char| {
            !c.is_whitespace() && c != '\n' && c != ',' && c != '}'
        }),
    ).map(|(name, _, _, _, ty_name)| {
        ast::Field {
            ident: ast::Ident(name),
            ty: ast::Ident(ty_name),
        }
    })
}
pub fn field_test<I>() -> impl Parser<Input = I, Output = u32>
where
    I: Stream<Item = char>,
{
    (
        many1::<String, _>(satisfy(|c: char| !c.is_whitespace() && c != ':')),
        spaces(),
        token(':'),
        spaces(),
        many1::<String, _>(satisfy(|c: char| {
            !c.is_whitespace() && c != '\n' && c != ',' && c != '}'
        })),
    ).map(|(name, _, _, _, ty_name)| 1)
}
pub fn fields_test<I>() -> impl Parser<Input = I, Output = Vec<u32>>
where
    I: Stream<Item = char>,
{
    sep_end_by(field_test(), token(',').skip(whitespace()))
}

pub fn digits<I>() -> impl Parser<Input = I, Output = Vec<char>>
where
    I: Stream<Item = char>,
{
    sep_end_by(digit().skip(spaces()), token(',').skip(spaces()))
}

pub fn fields<'a, I>() -> impl Parser<Input = I, Output = Vec<ast::Field<'a>>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    sep_end_by(field(), token(',').skip(whitespace()))
}

pub fn extension_with_digits<'a, I>() -> impl Parser<Input = I, Output = ast::Extension<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    (
        spaces(),
        string("extension").skip(spaces()),
        take_while(|c: char| !c.is_whitespace() && c != '{').skip(spaces()),
        token('{').skip(whitespace()),
        digits().skip(whitespace()),
        token('}').skip(whitespace()),
    ).map(|(_, _, name, _, fields, _)| {
        ast::Extension {
            ident: ast::Ident(name),
            fields: Vec::new(),
        }
    })
}
pub fn extension<'a, I>() -> impl Parser<Input = I, Output = ast::Extension<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
{
    (
        spaces(),
        string("extension").skip(spaces()),
        take_while(|c: char| !c.is_whitespace() && c != '{').skip(spaces()),
        token('{').skip(whitespace()),
        fields().skip(whitespace()),
        token('}').skip(whitespace()),
    ).map(|(_, _, name, _, fields, _)| {
        ast::Extension {
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
    let ast_extension = extension().map(|e| ast::Ast::Extension(e));
    many(ast_object.or(ast_extension).skip(whitespace()))
}

//pub fn wbl<'a, I>() -> impl Parser<Input = I, Output = Vec<ast::Ast<'a>>>
//    where
//    I: RangeStream<Item = char, Range = &'a str>,
//{
//}
