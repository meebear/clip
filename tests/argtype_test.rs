#[macro_use]
extern crate clip;

use clip::{ArgType, TrCustom};

#[test]
fn test_argtype_builtin() {
    let var = ArgType::Text(Some("helo".to_string()));
    assert_eq!(clip_value!(&var, Text), "helo");

    let var = ArgType::Text(None);
    assert_eq!(clip_value!(&var, Text), "");

    let var = ArgType::Bool(true);
    assert_eq!(clip_value!(&var, Bool), true);

    let var = ArgType::Count(1001);
    assert_eq!(clip_value!(&var, Count), 1001);
}

#[test]
#[should_panic]
fn test_argtype_unmatch() {
    let var = ArgType::Int64(100);
    assert_eq!(clip_value!(&var, Uint64), 100);
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
