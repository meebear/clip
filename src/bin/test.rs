extern crate clip;
use clip::{ArgType, Parser};

fn main() {
    let mut parser = Parser::new();

    parser.add_option(&["-n", "--name"], ArgType::Text(None)).required();
    parser.add_option(&["-j", "--jump"], ArgType::BoolFlag(false));
    parser.add_option(&["-v"], ArgType::IncFlag(0));
    parser.add_argument("src", ArgType::Text(None));
    parser.add_argument("dst", ArgType::Text(None));

    parser.dump();

    if let Err(e) = parser.parse() {
        println!("{}", e);
    }
}
