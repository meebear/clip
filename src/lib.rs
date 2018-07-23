
pub trait TrCustom: argtype::AsAny {
    fn parse_args(&mut self, vals: &[&str]) -> Result<(), String>;
}

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

pub enum ArgNum {
    NoArg,
    SingleArg,
    MultiArgs,
}

pub mod argtype;
mod parser;

