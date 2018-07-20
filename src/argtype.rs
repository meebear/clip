use super::ArgType;
use super::ArgType::{Text, Int64, Uint64, Bool, Count, Custom};

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
macro_rules! clip_value_ {
    ($at:ident, $dft:expr, $val:expr) => { {
        let mut v = ArgType::$at($dft);
        v.get_value($val);
        if let ArgType::$at(c) = v {
            Some(c)
        } else {
            None
        }
    } }
}

#[macro_export]
macro_rules! clip_value {
    (Text, $val:expr) => {
        clip_value_!(Text, None, $val).unwrap().unwrap_or_else(|| "".to_string())
    };
    (Bool, $val:expr) => {
        clip_value_!(Bool, false, $val).unwrap()
    };
    (Custom, $val:expr) => {
        panic!("TODO");
    };
    ($at:ident, $val:expr) => {
        clip_value_!($at, 0, $val).unwrap()
    };
}
