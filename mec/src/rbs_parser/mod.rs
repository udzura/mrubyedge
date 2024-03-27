extern crate nom;

#[derive(Debug)]
pub struct FuncDef {
    pub name: String,
    pub argstype: Vec<String>,
    pub rettype: String,
}

impl FuncDef {
    pub fn args_decl(&self) -> &str {
        if self.argstype.len() == 0 {
            return "";
        }

        match self.argstype.get(0) {
            Some(v) => match v.as_str() {
                "Integer" => "a: i32",
                _ => {
                    unimplemented!("unsupported arg type")
                }
            },
            _ => {
                unreachable!("to be checked argsize")
            }
        }
    }

    pub fn args_let_vec(&self) -> &str {
        if self.argstype.len() == 0 {
            return "vec![]";
        }

        match self.argstype.get(0) {
            Some(v) => match v.as_str() {
                "Integer" => "vec![std::rc::Rc::new(RObject::RInteger(a as i64))]",
                _ => {
                    unimplemented!("unsupported arg type")
                }
            },
            _ => {
                unreachable!("to be checked argsize")
            }
        }
    }

    pub fn rettype_decl(&self) -> &str {
        match self.rettype.as_str() {
            "void" => "-> ()",
            "Integer" => "-> i32",
            _ => {
                unimplemented!("unsupported arg type")
            }
        }
    }
}

use nom::branch::alt;
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::character::complete::*;
// use nom::combinator::opt;
use nom::error::context;
use nom::error::VerboseError;
use nom::multi::*;
use nom::sequence::tuple;
use nom::IResult;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn def(input: &str) -> Res<&str, ()> {
    context("def", tag("def"))(input).map(|(s, _)| (s, ()))
}

fn alpha_just_1(input: &str) -> Res<&str, char> {
    satisfy(|c| c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'))(input)
}

fn alphanumeric_just_1(input: &str) -> Res<&str, char> {
    satisfy(|c| {
        c == '_' || ('0' <= c && c <= '9') || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
    })(input)
}

fn symbol(input: &str) -> Res<&str, String> {
    tuple((alpha_just_1, many0(alphanumeric_just_1)))(input).map(|(s, (head, tail))| {
        let mut name: String = head.to_string();
        for c in tail.iter() {
            name += &c.to_string()
        }
        (s, name)
    })
}

fn method(input: &str) -> Res<&str, String> {
    tuple((symbol, char(':'), space0))(input).map(|(s, (sym, _, _))| (s, sym))
}

fn emptyarg(input: &str) -> Res<&str, Vec<String>> {
    tuple((char('('), space0, char(')')))(input).map(|(s, _)| (s, vec![]))
}

fn contentarg(input: &str) -> Res<&str, Vec<String>> {
    tuple((
        char('('),
        space0,
        symbol,
        space0,
        many0(tuple((char(','), space0, symbol, space0))),
        char(')'),
    ))(input)
    .map(|(s, (_, _, head, _, rest, _))| {
        let mut syms: Vec<String> = rest.into_iter().map(|(_, _, val, _)| val).collect();
        syms.insert(0, head);
        (s, syms)
    })
}

fn arg(input: &str) -> Res<&str, Vec<String>> {
    alt((emptyarg, contentarg))(input)
}

fn ret(input: &str) -> Res<&str, String> {
    tuple((tag("->"), space0, symbol))(input).map(|(s, (_, _, sym))| (s, sym))
}

fn fntype(input: &str) -> Res<&str, (Vec<String>, String)> {
    tuple((arg, space0, ret))(input).map(|(s, (arg, _, ret))| (s, (arg, ret)))
}

pub fn fn_def(input: &str) -> Res<&str, FuncDef> {
    tuple((def, space1, method, fntype))(input).map(|(s, (_, _, name, (argstype, rettype)))| {
        (
            s,
            FuncDef {
                name,
                argstype,
                rettype,
            },
        )
    })
}

pub fn parse(input: &str) -> Res<&str, Vec<FuncDef>> {
    tuple((
        multispace0,
        separated_list0(permutation((space0, many1(char('\n')), space0)), fn_def),
    ))(input)
    .map(|(s, (_, list))| (s, list))
}
