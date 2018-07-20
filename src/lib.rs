
pub trait TrCustom: argtype::AsAny {}

pub enum ArgType {
    Text(Option<String>),
    Int64(i64),
    Uint64(u64),
    Bool(bool),
    Count(i64),
    Custom(Box<dyn TrCustom>),
}

pub mod argtype;

