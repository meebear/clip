#[macro_use]
extern crate clip;

use clip::{ArgType, TrCustom};

#[test]
fn test_argtype_builtin() {
    let var = ArgType::Text(Some("helo".to_string()));
    assert_eq!(clip_value!(&var, Text), "helo");

    let var = ArgType::Text(None);
    assert_eq!(clip_value!(&var, Text), "");

    let var = ArgType::BoolFlag(true);
    assert_eq!(clip_value!(&var, BoolFlag), true);

    let var = ArgType::IncFlag(1001);
    assert_eq!(clip_value!(&var, IncFlag), 1001);
}

#[test]
fn test_argtype_array_builtin() {
    let var = ArgType::Texts(Some(vec!["helo".to_string(), "wold".to_string()]));
    assert_eq!(clip_value!(&var, Texts), vec!["helo", "wold"]);

    let var = ArgType::Ints(Some(vec![1, 2, 3]));
    assert_eq!(clip_value!(&var, Ints), vec![1, 2, 3]);

    let var = ArgType::Floats(Some(vec![3.14, 9.00]));
    assert_eq!(clip_value!(&var, Floats), vec![3.14, 9.00]);
}

#[test]
#[should_panic]
fn test_argtype_unmatch() {
    let var = ArgType::Int(100);
    assert_eq!(clip_value!(&var, Float), 100.0);
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
    let v: &Udata = clip_value!(&c, Custom);
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
    assert_eq!(clip_value!(&var, Ints), vec![12, 23, 34]);
}
