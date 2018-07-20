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
#[should_panic]
fn test_argtype_unmatch() {
    let var = ArgType::Int(100);
    assert_eq!(clip_value!(&var, Float), 100.0);
}

#[derive(PartialEq, Debug)]
struct Udata(i32, String);
impl TrCustom for Udata { }

#[test]
fn test_argtype_custom() {
    let c = ArgType::Custom(Box::new(Udata(216, "word".to_string())));
    let v: &Udata = clip_value!(&c, Custom);
    assert_eq!(*v, Udata(216, "word".to_string()));
}
