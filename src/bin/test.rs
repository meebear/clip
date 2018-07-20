#[macro_use]
extern crate mytest;

use mytest::argtype::ArgOpt;
use mytest::argtype::ArgType;
use mytest::argtype::ArgType::{Text, Bool, Int64};

fn main() {
    let itm = ArgOpt { var: ArgType::Text(Some("helo".to_string())) };
    println!("{}", get_value!(Text, &itm.var));

    let itm = ArgOpt { var: ArgType::Bool(true) };
    println!("{}", get_value!(Bool, &itm.var));

    let itm = ArgOpt { var: ArgType::Count(1001) };
    println!("{}", get_value!(Int64, &itm.var));
}
