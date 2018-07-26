#[macro_use]
extern crate clip;

use clip::{ArgType, TrCustom};

#[test]
fn test_argtype_builtin() {
    let var = ArgType::Text(Some("helo".to_string()));
    assert_eq!(clip_value_at!(&var, Text), Some("helo".to_string()));

    let var = ArgType::Text(None);
    assert_eq!(clip_value_at!(&var, Text), None);

    let var = ArgType::BoolFlag(Some(true));
    assert_eq!(clip_value_at!(&var, BoolFlag), Some(true));

    let var = ArgType::IncFlag(Some(1001));
    assert_eq!(clip_value_at!(&var, IncFlag), Some(1001));

    let var = ArgType::Float(Some(100.1));
    assert_eq!(clip_value_at!(&var, Float), Some(100.1));

    let var = ArgType::Int(Some(102));
    assert_eq!(clip_value_at!(&var, Int), Some(102));
}

#[test]
fn test_argtype_array_builtin() {
    let var = ArgType::Texts(Some(vec!["helo".to_string(), "wold".to_string()]));
    assert_eq!(clip_value_at!(&var, Texts), Some(vec!["helo".to_string(), "wold".to_string()]));

    let var = ArgType::Ints(Some(vec![1, 2, 3]));
    assert_eq!(clip_value_at!(&var, Ints), Some(vec![1, 2, 3]));

    let var = ArgType::Floats(Some(vec![3.14, 9.00]));
    assert_eq!(clip_value_at!(&var, Floats), Some(vec![3.14, 9.00]));
}

#[test]
#[should_panic]
fn test_argtype_unmatch() {
    let var = ArgType::Int(Some(100));
    assert_eq!(clip_value_at!(&var, Float), Some(100.0));
}

#[derive(PartialEq, Debug)]
struct Udata(i32, String);
impl TrCustom for Udata {
    fn parse_args(&mut self, _vals: &[&str]) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn test_argtype_custom() {
    let c = ArgType::Custom(Box::new(Udata(216, "word".to_string())));
    let v: &Udata = clip_value_at!(&c, Custom);
    assert_eq!(*v, Udata(216, "word".to_string()));
}

#[test]
fn test_argtype_set_value() {
    let mut var = ArgType::Ints(None);
    match var.set_value(&["12", "23", "34"]) {
        Ok(_) => {
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
    assert_eq!(clip_value_at!(&var, Ints), Some(vec![12, 23, 34]));
}
