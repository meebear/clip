
pub trait TrCustom: argtype::AsAny {}

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

pub mod argtype;

