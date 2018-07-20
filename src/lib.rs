
pub trait TrCustom: argtype::AsAny {}

pub enum ArgType {
    Text(Option<String>),
    Int(i64),
    Float(f64),
    BoolFlag(bool),
    IncFlag(usize),
    Custom(Box<dyn TrCustom>),
}

pub mod argtype;

