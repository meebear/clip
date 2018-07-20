use std::any::Any;

use super::ArgType;
use super::ArgType::{Text, Int, Float, BoolFlag, IncFlag, Custom};
use super::TrCustom;

pub trait AsAny {
    fn as_any(&self) -> &Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any { self }
}

impl ArgType {
    /*
    fn from_str(&mut self, _s: &str) -> Result<(), String> {
        Ok(())
    }
    */

    pub fn get_value(&self, to: &mut ArgType) {
        match (to, self) {
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
            (Int(v), Int(f))   => { *v = *f; },
            (Float(v), Float(f)) => { *v = *f; },
            (BoolFlag(v), BoolFlag(f))     => { *v = *f; },
            (IncFlag(v), IncFlag(f))   => { *v = *f; },
            (Custom(_), _) => {
                panic!("use get_custom_value() for custom type");
            }
            _ => {
                panic!("unmatched argument type")
            },
        }
    }

    pub fn get_custom_value<T: TrCustom + 'static>(&self) -> Option<&T> {
        match self {
            Custom(c) => {
                if let Some(value) = (**c).as_any().downcast_ref::<T>() {
                    Some(value)
                } else {
                    panic!("unmatched custom type"); 
                }
            },
            _ => {
                panic!("not custom type");
            }
        }
    }
}

#[macro_export]
macro_rules! clip_value_ {
    ($at:ident, $dft:expr, $val:expr) => { {
        let mut v = ArgType::$at($dft);
        $val.get_value(&mut v);
        if let ArgType::$at(c) = v {
            Some(c)
        } else {
            None
        }
    } }
}

#[macro_export]
macro_rules! clip_value {
    ($val:expr, Text) => {
        clip_value_!(Text, None, $val).unwrap().unwrap_or_else(|| "".to_string())
    };
    ($val:expr, BoolFlag) => {
        clip_value_!(BoolFlag, false, $val).unwrap()
    };
    ($val:expr, Float) => {
        clip_value_!(Float, 0.0, $val).unwrap()
    };
    ($val:expr, Custom) => {
        $val.get_custom_value().unwrap()
    };
    ($val:expr, $at:ident) => {  /* all integer types */
        clip_value_!($at, 0, $val).unwrap()
    };
}
