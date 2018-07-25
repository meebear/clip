use std::fmt::Debug;
use std::collections::HashMap;

pub trait TrCustom: argtype::AsAny + Debug {
    fn parse_args(&mut self, vals: &[&str]) -> Result<(), String>;
}

#[derive(Debug)]
pub enum ArgType {
    BoolFlag(bool),
    IncFlag(usize),

    Text(Option<String>),
    Int(i64),
    Float(f64),

    Texts(Option<Vec<String>>),
    Ints(Option<Vec<i64>>),
    Floats(Option<Vec<f64>>),

    Custom(Box<dyn TrCustom>),
}

#[derive(Clone, Copy, Debug)]
pub enum ArgNum {
    NoArg,
    SingleArg,
    MultiArgs,
}

pub struct Parser {
    curr: Option<parser::ArgIdx>,

    opts: Vec<parser::ArgOpt>,

    args: Vec<parser::ArgOpt>,
    next_arg: usize,

    index: HashMap<String, parser::ArgIdx>,
}

pub mod argtype;
pub mod parser;
