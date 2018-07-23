use std::collections::HashMap;
use std::collections::hash_map::Entry;
use super::{ArgType, ArgNum, TrCustom};
use self::ArgKind::{ShortOption, LongOption, Positional, Delimiter};

enum ArgKind {
    ShortOption,
    LongOption,
    Positional,
    Delimiter,
}

impl ArgKind {
    fn check(name: &str) -> ArgKind {
        let mut iter = name.chars();
        match iter.next() {
            Some('-') => match iter.next() {
                Some('-') => match iter.next() {
                    Some(_) => LongOption,
                    None => Delimiter,
                },
                Some(_) => ShortOption,
                None => Positional,
            },
            Some(_) | None => Positional,
        }
    }
}

pub struct ArgOpt {
    var: ArgType,
    required: bool,
}

struct ArgIdx {
    idx: usize,
    argnum: ArgNum,
}

pub struct Parser {
    opts: Vec<ArgOpt>,
    args: Vec<ArgOpt>,
    index: HashMap<String, ArgIdx>,
}


impl Parser {
    pub fn new() -> Self {
        Parser {
            opts: vec![],
            args: vec![],
            index: HashMap::new(),
        }
    }

    fn add_option_(&mut self, opnames: &[&str], var: ArgType, argnum: ArgNum)
        -> &mut ArgOpt {
        let argopt = ArgOpt {
            var: var,
            required: false,
        };
        let varid = self.opts.len();
        self.opts.push(argopt);

        for opname in opnames {
            match ArgKind::check(opname) {
                ShortOption | LongOption => {
                    match self.index.entry(opname.to_string()) {
                        Entry::Occupied(_) => {
                            panic!("option {} already defined");
                        },
                        Entry::Vacant(vac) => {
                            vac.insert(ArgIdx {idx: varid, argnum: argnum});
                        }
                    };
                },
                Positional | Delimiter => {
                    panic!("none option!");
                },
            }
        }
        self.opts.last_mut().unwrap()
    }

    pub fn add_option(&mut self, opnames: &[&str], var: ArgType) -> &mut ArgOpt {
        let argnum = match var {
            ArgType::BoolFlag(_) | ArgType::IncFlag(_)
                => ArgNum::NoArg,
            ArgType::Text(_) | ArgType::Int(_) | ArgType::Float(_)
                => ArgNum::SingleArg,
            ArgType::Texts(_) | ArgType::Ints(_) | ArgType::Floats(_)
                => ArgNum::MultiArgs,
            ArgType::Custom(_) => panic!("custom"),
        };
        self.add_option_(opnames, var, argnum)
    }

    pub fn add_custom_option(&mut self, opnames: &[&str],
                             var: Box<dyn TrCustom>, argnum: ArgNum) -> &mut ArgOpt {
        self.add_option_(opnames, ArgType::Custom(var), argnum)
    }

    fn add_argument_(&mut self, name: &str, var: ArgType, argnum: ArgNum) -> &mut ArgOpt {
        let argopt = ArgOpt {
            var: var,
            required: false,
        };
        let varid = self.args.len();
        self.args.push(argopt);

        match self.index.entry(name.to_string()) {
            Entry::Occupied(_) => {
                panic!("option {} already defined");
            },
            Entry::Vacant(vac) => {
                vac.insert(ArgIdx {idx: varid, argnum: argnum});
            }
        };

        self.args.last_mut().unwrap()
    }

    pub fn add_argument(&mut self, name: &str, var: ArgType) -> &mut ArgOpt {
        let argnum = match var {
            ArgType::BoolFlag(_) | ArgType::IncFlag(_)
                => ArgNum::NoArg,
            ArgType::Text(_) | ArgType::Int(_) | ArgType::Float(_)
                => ArgNum::SingleArg,
            ArgType::Texts(_) | ArgType::Ints(_) | ArgType::Floats(_)
                => ArgNum::MultiArgs,
            ArgType::Custom(_) => panic!("custom"),
        };
        self.add_argument_(name, var, argnum)
    }

    pub fn add_custom_argument(&mut self, name: &str, var: ArgType, argnum: ArgNum)
        -> &mut ArgOpt {
        self.add_argument_(name, var, argnum)
    }
}
