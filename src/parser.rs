use std::env;
use std::collections::hash_map::Entry;
use super::{Parser, ArgType, ArgNum, TrCustom};
use self::ArgKind::{ShortOption, LongOption, Positional, Delimiter};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};

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

impl Display for ArgNum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            ArgNum::NoArg => write!(f, "no argument"),
            ArgNum::SingleArg => write!(f, "single argument"),
            ArgNum::MultiArgs => write!(f, "multiple arguments"),
        }
    }
}

#[derive(Debug)]
pub struct ArgOpt {
    var: ArgType,
    required: bool,
    argnum: ArgNum,
}

#[derive(Debug)]
pub struct ArgIdx {
    idx: usize,
}

pub struct Curr {
    varid: usize,
    is_opt: bool,
}


impl Parser {
    pub fn new() -> Self {
        Parser {
            curr: None,
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
            argnum: argnum,
        };
        let is_opt = true;
        let varid = self.opts.len();
        self.curr = Some(Curr{varid, is_opt});
        self.opts.push(argopt);

        for opname in opnames {
            match ArgKind::check(opname) {
                ShortOption | LongOption => {
                    match self.index.entry(opname.to_string()) {
                        Entry::Occupied(_) => {
                            panic!("option {} already defined");
                        },
                        Entry::Vacant(vac) => {
                            vac.insert(ArgIdx {idx: varid });
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
            argnum: argnum,
        };
        let is_opt = true;
        let varid = self.opts.len();
        self.curr = Some(Curr{varid, is_opt});
        self.args.push(argopt);

        match self.index.entry(name.to_string()) {
            Entry::Occupied(_) => {
                panic!("option {} already defined");
            },
            Entry::Vacant(vac) => {
                vac.insert(ArgIdx {idx: varid});
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
        match &self.curr {
            Some(c) => {
                if c.is_opt {
                    &mut self.opts[c.varid]
                } else {
                    &mut self.args[c.varid]
                }
            },
            None => {
                panic!("no current option set");
            }
        }
    }

    pub fn required(&mut self) -> &mut Self {
        self.get_curr().required = true;
        self
    }

    pub fn dump(&self) {
        println!("Registered-Options:");
        for (i, o) in self.opts.iter().enumerate() {
            println!(" {}: {:?}", i, o);
        }
        println!("\nRegistered-Positional-Arguments:");
        for (i, a) in self.args.iter().enumerate() {
            println!(" {}: {:?}", i, a);
        }
        println!("\nGenerated Index:");
        for (k, v) in self.index.iter() {
            println!(" {:?}: {:?}", k, v);
        }
        println!("------PARSE--------\n");
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let mut args = env::args();
        args.next(); // skip the program name
        loop {
            match args.next() {
                Some(s) => {
                    match ArgKind::check(&s) {
                        LongOption => { self.parse_long_option(&s, &mut args)? },
                        ShortOption => { self.parse_short_options(&s, &args)? },
                        Positional => { self.parse_argument(&s, &args)? },
                        Delimiter => { self.parse_delimiter(&args)? }
                    };
                },
                None => { break; },
            }
        }
        Ok(())
    }

    fn parse_long_option(&mut self, name: &str, args: &mut env::Args) -> Result<(), String> {
        let mut iter = name.splitn(2, '=');
        let opname = iter.next().unwrap();
        let valref = iter.next();
        if let Some(ix) = self.index.get(opname) {
            let argopt = &self.opts[ix.idx];
            match argopt.argnum {
                ArgNum::MultiArgs | ArgNum::SingleArg => {
                    if let Some(val) = valref {
                        println!("Set '{}' = '{}'", opname, val);
                    } else {
                        if let Some(val) = args.next() {
                            println!("Set '{}' = '{}'", opname, val);
                        } else {
                            return Err(format!("Option '{}' expects {}",
                                               opname, argopt.argnum));
                        }
                    }
                },
                ArgNum::NoArg => {
                    if let Some(_) = valref {
                        return Err(format!("Option '{}' takes no argument", opname));
                    }
                },
            }
            Ok(())
        } else {
            Err(format!("Option '{}' not recognized", opname))
        }
    }

    fn parse_short_options(&mut self, opt: &str, args: &env::Args) -> Result<(), String> {
        Ok(())
    }

    fn parse_argument(&mut self, opt: &str, args: &env::Args) -> Result<(), String> {
        Ok(())
    }

    fn parse_delimiter(&mut self, args: &env::Args) -> Result<(), String> {
        Ok(())
    }
}
