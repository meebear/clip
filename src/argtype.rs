use std::any::Any;

use super::TrCustom;
use super::ArgType;
use super::ArgType::{Text, Int, Float, BoolFlag, IncFlag, Custom, Texts, Ints, Floats};

pub trait AsAny {
    fn as_any(&self) -> &Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any { self }
}

macro_rules! get_option_value {
    ($v:ident, $f:ident) => {
        if let Some(fs) = $f {
            if let Some(vs) = $v {
                vs.clone_from(fs);
            } else {
                *$v = Some(fs.clone());
            }
        } else {
            if let Some(_) = $v {
                *$v = None;
            }
        }
    }
}

impl ArgType {
    pub fn get_value(&self, to: &mut ArgType) {
        match (to, self) {
            (Text(v), Text(f))         => { get_option_value!(v, f); },
            (Texts(v), Texts(f))       => { get_option_value!(v, f); },
            (Ints(v), Ints(f))         => { get_option_value!(v, f); },
            (Floats(v), Floats(f))     => { get_option_value!(v, f); },
            (Int(v), Int(f))           => { *v = *f; },
            (Float(v), Float(f))       => { *v = *f; },
            (BoolFlag(v), BoolFlag(f)) => { *v = *f; },
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

    pub fn set_value(&mut self, vals: &[&str]) -> Result<(), String> {
        match self {
            BoolFlag(v) => {
                *v = true;
            },
            IncFlag(v) => {
                *v += 1;
            },
            Int(v) => {
                match vals[0].parse::<i64>() {
                    Ok(n) => { *v = n; },
                    Err(e) => return Err(e.to_string()),
                }
            },
            Float(v) => {
                match vals[0].parse::<f64>() {
                    Ok(n) => { *v = n; },
                    Err(e) => return Err(e.to_string()),
                }
            },
            Text(v) => {
                *v = Some(vals[0].to_string());
            },
            Ints(v) => {
                let mut ints = vec![];
                for s in vals.iter() {
                    match s.parse::<i64>() {
                        Ok(n) => ints.push(n),
                        Err(e) => return Err(e.to_string()),
                    }
                }
                *v = Some(ints);
            },
            Floats(v) => {
                let mut floats = vec![];
                for s in vals.iter() {
                    match s.parse::<f64>() {
                        Ok(n) => floats.push(n),
                        Err(e) => return Err(e.to_string()),
                    }
                }
                *v = Some(floats);
            },
            Texts(v) => {
                let mut texts = vec![];
                for s in vals.iter() {
                    texts.push(s.to_string());
                }
                *v = Some(texts);
            },
            Custom(v) => {
                return v.parse_args(vals);
            }
        }
        Ok(())
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
    ($val:expr, Texts) => {
        clip_value_!(Texts, None, $val).unwrap().unwrap_or_else(|| vec![])
    };
    ($val:expr, Ints) => {
        clip_value_!(Ints, None, $val).unwrap().unwrap_or_else(|| vec![])
    };
    ($val:expr, Floats) => {
        clip_value_!(Floats, None, $val).unwrap().unwrap_or_else(|| vec![])
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
    // Int, IncFlag
    ($val:expr, $at:ident) => {  /* all integer types */
        clip_value_!($at, 0, $val).unwrap()
    };
}
