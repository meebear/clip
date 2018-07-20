use std::any::Any;

pub enum ArgType {
    Text(Option<String>),
    Int64(i64),
    Uint64(u64),
    Bool(bool),
    Count(i64),
    Custom(Box<dyn Any>),
}

use self::ArgType::{Text, Int64, Uint64, Bool, Count, Custom};

pub struct ArgOpt {
    pub var: ArgType,
}

impl ArgType {
    /*
    fn from_str(&mut self, _s: &str) -> Result<(), String> {
        Ok(())
    }
    */

    pub fn get_value(&mut self, from: &ArgType) {
        match (self, from) {
            (Text(v), Text(f))   => {
                if let Some(fs) = f {
                    if let Some(vs) = v {
                        vs.clone_from(fs);
                    } else {
                        *v = Some(fs.clone());
                    }
                } else {
                    if let Some(_) = v {
                        *v = None;
                    }
                }
            },
            (Int64(v), Int64(f))   => { *v = *f; },
            (Uint64(v), Uint64(f)) => { *v = *f; },
            (Bool(v), Bool(f))     => { *v = *f; },
            (Count(v), Count(f))   => { *v = *f; },
            (Custom(_), _) => {
                panic!("use get_custom_value!() for custom type");
            }
            _ => {
                panic!("unmatched argument type")
            },
        }
    }
}

#[macro_export]
macro_rules! get_value_ {
    ($at:ident, $dft:expr, $val:expr) => { {
        let mut v = $at($dft);
        v.get_value($val);
        if let $at(c) = v {
            Some(c)
        } else {
            None
        }
    } }
}

#[macro_export]
macro_rules! get_value {
    (Text, $val:expr) => {
        get_value_!(Text, None, $val).unwrap().unwrap_or_else(|| "".to_string())
    };
    (Bool, $val:expr) => {
        get_value_!(Bool, false, $val).unwrap()
    };
    (Custom, $val:expr) => {
        panic!("TODO");
    };
    ($at:ident, $val:expr) => {
        get_value_!($at, 0, $val).unwrap()
    };
}
