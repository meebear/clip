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

impl ArgNum {
    fn check(var: &ArgType) -> Self {
        match var {
            ArgType::BoolFlag(_) | ArgType::IncFlag(_)
                => ArgNum::NoArg,
            ArgType::Text(_) | ArgType::Int(_) | ArgType::Float(_)
                => ArgNum::SingleArg,
            ArgType::Texts(_) | ArgType::Ints(_) | ArgType::Floats(_)
                => ArgNum::MultiArgs,
            ArgType::Custom(_) => panic!("custom"),
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

struct Curr {
    varid: usize,
    is_opt: bool,
}

pub struct Parser {
    curr: Curr,

    opts: Vec<ArgOpt>,
    args: Vec<ArgOpt>,
    index: HashMap<String, ArgIdx>,
}


impl Parser {
    pub fn new() -> Self {
        Parser {
            curr: Curr{varid: 0, is_opt: false},
            opts: vec![],
            args: vec![],
            index: HashMap::new(),
        }
    }

    fn add_option_(&mut self, opnames: &[&str], var: ArgType, argnum: ArgNum)
        -> &mut Self {
        let argopt = ArgOpt {
            var: var,
            required: false,
        };
        self.curr.is_opt = true;
        self.curr.varid = self.opts.len();
        self.opts.push(argopt);

        for opname in opnames {
            match ArgKind::check(opname) {
                ShortOption | LongOption => {
                    match self.index.entry(opname.to_string()) {
                        Entry::Occupied(_) => {
                            panic!("option {} already defined");
                        },
                        Entry::Vacant(vac) => {
                            vac.insert(ArgIdx {idx: self.curr.varid, argnum: argnum});
                        }
                    };
                },
                Positional | Delimiter => {
                    panic!("none option!");
                },
            }
        }
        self
    }

    pub fn add_option(&mut self, opnames: &[&str], var: ArgType) -> &mut Self {
        let argnum = ArgNum::check(&var);
        self.add_option_(opnames, var, argnum)
    }

    pub fn add_custom_option(&mut self, opnames: &[&str],
                             var: Box<dyn TrCustom>, argnum: ArgNum) -> &mut Self {
        self.add_option_(opnames, ArgType::Custom(var), argnum)
    }

    fn add_argument_(&mut self, name: &str, var: ArgType, argnum: ArgNum) -> &mut Self {
        let argopt = ArgOpt {
            var: var,
            required: false,
        };
        self.curr.is_opt = false;
        self.curr.varid = self.args.len();
        self.args.push(argopt);

        match self.index.entry(name.to_string()) {
            Entry::Occupied(_) => {
                panic!("option {} already defined");
            },
            Entry::Vacant(vac) => {
                vac.insert(ArgIdx {idx: self.curr.varid, argnum: argnum});
            }
        };
        self
    }

    pub fn add_argument(&mut self, name: &str, var: ArgType) -> &mut Parser {
        let argnum = ArgNum::check(&var);
        self.add_argument_(name, var, argnum)
    }

    pub fn add_custom_argument(&mut self, name: &str, var: ArgType, argnum: ArgNum)
        -> &mut Self {
        self.add_argument_(name, var, argnum)
    }

    fn get_curr(&mut self) -> &mut ArgOpt {
        if self.curr.is_opt {
            &mut self.opts[self.curr.varid]
        } else {
            &mut self.args[self.curr.varid]
        }
    }

    pub fn required(&mut self) -> &mut Self {
        self.get_curr().required = true;
        self
    }
}
