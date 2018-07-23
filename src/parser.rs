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
        let char1 = iter.next();
        let char2 = iter.next();
        let char3 = iter.next();
        match char1 {
            Some('-') => match char2 {
                Some('-') => match char3 {
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

struct ArgOpt {
    var: ArgType,
    required: bool,
}

struct ArgIdx {
    idx: usize,
    argnum: ArgNum,
}

struct Parser {
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

    pub fn add_option_(&mut self, opnames: &[&str], var: ArgType, argnum: ArgNum) -> &mut ArgOpt {
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
        //&mut self.opts[varid]
    }

    pub fn add_option(&mut self, opnames: &[&str], var: ArgType) -> &mut ArgOpt {
        let argnum = match var {
            ArgType::BoolFlag(_) | ArgType::IncFlag(_) => ArgNum::NoArg,
            ArgType::Text(_) | ArgType::Int(_) | ArgType::Float(_) => ArgNum::SingleArg,
            ArgType::Texts(_) | ArgType::Ints(_) | ArgType::Floats(_) => ArgNum::MultiArgs,
            ArgType::Custom(_) => panic!("custom"),
        };
        self.add_option_(opnames, var, argnum)
    }

    pub fn add_custom_option(&mut self, opnames: &[&str], var: Box<dyn TrCustom>, argnum: ArgNum) -> &ArgOpt {
        self.add_option_(opnames, ArgType::Custom(var), argnum)
    }

/*
    pub fn add_positional(&mut self, name: &str, at: ArgType) -> &ArgOpt {
        &ArgOpt {
            var: ArgType::BoolFlag(false),
            required: false,
        }
    }

    pub fn add_custom_positional(&mut self, opnames: &[&str], argnum: ArgNum) -> &ArgOpt {
        &ArgOpt {
            var: ArgType::BoolFlag(false),
            required: false,
        }
    }
    */
}