#[macro_use]
extern crate clip;

use clip::{ArgType, TrCustom};

#[test]
fn test_argtype_builtin() {
    let var = ArgType::Text(Some("helo".to_string()));
    assert_eq!(clip_value!(Text, &var), "helo");

    let var = ArgType::Bool(true);
    assert_eq!(clip_value!(Bool, &var), true);

    let var = ArgType::Count(1001);
    assert_eq!(clip_value!(Count, &var), 1001);
}

#[test]
#[should_panic]
fn test_argtype_unmatch() {
    let var = ArgType::Int64(100);
    assert_eq!(clip_value!(Uint64, &var), 100);
}

#[derive(PartialEq, Debug)]
struct Udata(i32);
impl TrCustom for Udata { }

#[test]
fn test_argtype_custom() {
    let c = ArgType::Custom(Box::new(Udata(216)));
    let v: &Udata = clip_value!(Custom, &c);
    assert_eq!(*v, Udata(216));
}
