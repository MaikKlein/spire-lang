use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Ident<'a>(pub &'a str);

#[derive(Debug, Clone, Copy)]
pub enum Value<'a> {
    Text(&'a str),
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    pub ident: Ident<'a>,
    pub rvalue: RValue<'a>,
}
#[derive(Debug, Clone)]
pub struct Group<'a> {
    pub idents: Vec<Ident<'a>>,
}

pub struct Link;

#[derive(Debug, Clone)]
pub enum RValue<'a> {
    Group(Group<'a>),
    Value(Value<'a>),
}

#[derive(Debug)]
pub struct Object<'a> {
    pub ident: Ident<'a>,
    pub map: HashMap<Ident<'a>, RValue<'a>>,
}

use std::marker::PhantomData;
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Id<T>(usize, PhantomData<T>);
impl<T> Id<T> {
    pub fn new(id: usize) -> Self {
        Id(id, PhantomData)
    }
}
mod id {
    #[derive(Debug, Clone, Copy)]
    pub struct Extension;
}
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int,
    Float,
    String,
    Extension(Id<id::Extension>),
}
#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub ident: Ident<'a>,
    pub ty: Ident<'a>,
}

#[derive(Debug, Clone)]
pub struct Extension<'a> {
    pub ident: Ident<'a>,
    pub fields: Vec<Field<'a>>,
}
impl<'a> Object<'a> {
    pub fn new(ident: Ident<'a>) -> Self {
        Object {
            map: HashMap::new(),
            ident,
        }
    }
}

#[derive(Debug)]
pub enum Ast<'a>{
    Object(Object<'a>),
    Extension(Extension<'a>)
}
