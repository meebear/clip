use std::env;
use std::io;
use std::collections::hash_map::Entry;
use super::{Parser, ArgType, ArgNum, TrCustom};
use self::ArgKind::{ShortOption, LongOption, Positional, Delimiter};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use super::help::wrap_text;

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
    pub var: ArgType,
    required: bool,
    argnum: ArgNum,
}

#[derive(Debug)]
pub struct ArgIdx {
    idx: usize,
    is_opt: bool,
}


impl Parser {
    pub fn new() -> Self {
        Parser {
            curr: None,
            opts: vec![],
            args: vec![],
            next_arg: 0,
            index: HashMap::new(),
            about: vec![],
            subcmds: vec![],
        }
    }

    pub fn register_subcmd(&mut self, subcmd: Box<Parser>) {
        self.subcmds.push(subcmd); // this will copy the struct to heap..
    }

    pub fn about(&mut self, info: &str) {
        self.about.push(info.to_string());
    }

    fn add_option_(&mut self, opnames: &[&str], var: ArgType, argnum: ArgNum)
        -> &mut Self {
        let argopt = ArgOpt {
            var: var,
            required: false,
            argnum: argnum,
        };
        let varid = self.opts.len();
        self.curr = Some(ArgIdx{idx: varid, is_opt: true});
        self.opts.push(argopt);

        for opname in opnames {
            match ArgKind::check(opname) {
                ShortOption | LongOption => {
                    match self.index.entry(opname.to_string()) {
                        Entry::Occupied(_) => {
                            panic!("option {} already defined", opname);
                        },
                        Entry::Vacant(vac) => {
                            vac.insert(ArgIdx {idx: varid, is_opt: true});
                        }
                    };
                },
                Positional => {
                    panic!("non option");
                },
                Delimiter => {
                    panic!("delimiter -- cannot be added");
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
        if let ArgNum::NoArg = argnum {
            panic!("positional argument must take arguments");
        }
        if name == "--" {
            panic!("delimiter -- cannot be added");
        }
        let argopt = ArgOpt {
            var: var,
            required: false,
            argnum: argnum,
        };
        let varid = self.args.len();
        self.curr = Some(ArgIdx{idx: varid, is_opt: false});
        self.args.push(argopt);

        match self.index.entry(name.to_string()) {
            Entry::Occupied(_) => {
                panic!("option {} already defined", name);
            },
            Entry::Vacant(vac) => {
                vac.insert(ArgIdx {idx: varid, is_opt: false});
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
                    &mut self.opts[c.idx]
                } else {
                    &mut self.args[c.idx]
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
        self.next_arg = 0;
        let mut args = env::args();
        args.next(); // skip the program name
        loop {
            match args.next() {
                Some(s) => {
                    match ArgKind::check(&s) {
                        LongOption => { self.parse_long_option(&s, &mut args)? },
                        ShortOption => { self.parse_short_options(&s, &mut args)? },
                        Positional => { self.parse_argument(&s)? },
                        Delimiter => { self.parse_delimiter(&mut args)? }
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
            let argopt = &mut self.opts[ix.idx];
            match argopt.argnum {
                ArgNum::MultiArgs | ArgNum::SingleArg => {
                    if let Some(val) = valref {
                        println!("Set '{}' = '{}'", opname, val);
                        argopt.set_value(val)?;
                    } else {
                        if let Some(val) = args.next() {
                            println!("Set '{}' = '{}'", opname, val);
                            argopt.set_value(&val)?;
                        } else {
                            return Err(format!("Option '--{}' expects {}",
                                               opname, argopt.argnum));
                        }
                    }
                },
                ArgNum::NoArg => {
                    if let Some(_) = valref {
                        return Err(format!("Option '--{}' takes no argument", opname));
                    }
                },
            }
            Ok(())
        } else {
            Err(format!("Option '--{}' not recognized", opname))
        }
    }

    fn parse_short_options(&mut self, name: &str, args: &mut env::Args) -> Result<(), String> {
        let mut iter = name.char_indices().peekable();
        iter.next(); // to skip leading -
        loop {
            match iter.next() {
                Some((i, c)) => {
                    if let Some(ix) = self.index.get(&format!("-{}", c)) {
                        let argopt = &mut self.opts[ix.idx];
                        match argopt.argnum {
                            ArgNum::MultiArgs | ArgNum::SingleArg => {
                                let val = if let Some((ie, '=')) = iter.peek() {
                                          &name[ie+1..]
                                      } else {
                                          &name[i+1..]
                                      };

                                if val.len() > 0 {
                                    println!("Set '-{}' to {}", c, val);
                                    argopt.set_value(val)?;
                                } else {
                                    if let Some(val) = args.next() {
                                        println!("Set '-{}' = '{}'", c, val);
                                        argopt.set_value(&val)?;
                                    } else {
                                        return Err(format!("Option '-{}' expects {}",
                                                           c, argopt.argnum));
                                    }
                                }
                                break;
                            },
                            ArgNum::NoArg => {
                                argopt.set_value("")?;
                                println!("Set '-{}'", c);
                            },
                        }
                    } else {
                        return Err(format!("Option '-{}' not recognized", c))
                    }
                },
                None => { break; },
            }
        }
        Ok(())
    }

    fn parse_argument(&mut self, name: &str) -> Result<(), String> {
        if self.next_arg >= self.args.len() {
            panic!("extra argument '{}'", name);
        }
        let argopt = &mut self.args[self.next_arg];
        println!("Set positional {} = '{}'", self.next_arg, name);
        argopt.set_value(name)?;
        self.next_arg += 1;
        Ok(())
    }

    fn parse_delimiter(&mut self, args: &mut env::Args) -> Result<(), String> {
        match self.index.entry("--".to_string()) {
            Entry::Occupied(_) => { },
            Entry::Vacant(vac) => {
                let argopt = ArgOpt {
                    var: ArgType::Texts(None),
                    required: false,
                    argnum: ArgNum::MultiArgs,
                };
                let varid = self.opts.len();
                self.opts.push(argopt);
                vac.insert(ArgIdx {idx: varid, is_opt: true});
            }
        };
        if let Some(ix) = self.index.get("--") {
            let vals: Vec<String> = args.collect(); // TODO how to do this more effeciently?
            let vrefs: Vec<&str> = vals.iter().map(|s| &s[..]).collect();
            let argopt = &mut self.opts[ix.idx];
            argopt.var.set_value(&vrefs[..])?;
        }
        Ok(())
    }

    pub fn get_argopt(&self, key: &str) -> Option<&ArgOpt> {
        if let Some(ix) = self.index.get(key) {
            if ix.is_opt {
                Some(&self.opts[ix.idx])
            } else {
                Some(&self.args[ix.idx])
            }
        } else {
            None
        }
    }

    pub fn print_usage(&self) {
        for s in &self.about {
            if let Err(e) = wrap_text(&mut io::stdout(), s, 80, 0) {
                println!("{}", e);
                break;
            }
            println!();
            println!();
        }
    }
}

impl ArgOpt {
    fn set_value(&mut self, val: &str) -> Result<(), String> {
        self.var.set_value(&val.split(',').collect::<Vec<&str>>())
    }
}

#[macro_export]
macro_rules! clip_value {
    ($parser:ident, $key:expr, $at:ident) => {
        match $parser.get_argopt($key) {
            Some(argopt) => {
                clip_value_at!(argopt.var, $at)
            },
            None => {
                panic!("cannot find option by '{}'", stringify!($key));
            },
        }
    };
}
