#[macro_use]
extern crate lazy_static;

mod ls;
mod parser_options;
mod parse_file;

extern crate term;
extern crate getopts;
extern crate regex;
extern crate rustc_serialize;
extern crate toml;


use getopts::Options;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] ROOT_FOLDER", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("g", "git", "Search in files versionned in git");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") || matches.free.len() != 1 {
        print_usage(&program, opts);
        return;
    }

    let path = Path::new(&matches.free[0]);
    
    let mut config = String::new();
    let parsing_opts: parser_options::ParserOptions =
    if File::open("config.toml").and_then(|mut f| {
        f.read_to_string(&mut config)
    }).is_ok() {
        let mut toml_parser = toml::Parser::new(&(config.as_str()));
        let toml_value = match toml_parser.parse() {
            Some(value)  => {
                value
            }
            None => {
                panic!("Error parsing config: {:?}", toml_parser.errors);
            }
        };
        toml::decode(toml::Value::Table(toml_value)).unwrap()
    } else {
        parser_options::ParserOptions {
            default: parser_options::ParsingOptions {
                regex_server_value:  r"(?i)(\.|dabel69(\.corp\.altengroup\.dir)?)\\sqlexpress".to_string(),
                regex_wrong_database: r"^.*_...$".to_string(),
            }
        }
    };

    let files = if matches.opt_present("g") {
        ls::list_git_files(&path, &regex::Regex::new(r".*\.config$").unwrap())
    } else {
        ls::list_files(&path, &regex::Regex::new(r".*\.config$").unwrap())
    };

    let mut output = term::stdout().unwrap();
    for f in files.unwrap() {
        match parse_file::parse_file(&f, parse_file::Parser::DotNet, &parsing_opts.default) {
            Ok(list) => {
                for item in list {
                    if item.is_good {
                        output.fg(term::color::GREEN).unwrap();
                        writeln!(output, "File: {}; Name: {}; Server: {}; DB: {}",
                                           f,
                                           item.cs_name,
                                           item.server_name,
                                           item.db_name).unwrap();
                    } else {
                        output.fg(term::color::RED).unwrap();
                    }
                    writeln!(output, "File: {}; Name: {}; Server: {}; DB: {}",
                                        f,
                                        item.cs_name,
                                        item.server_name,
                                        item.db_name).unwrap();
                }
            }
            Err(err) => {
                output.fg(term::color::YELLOW).unwrap();
                writeln!(output, "File: {}; Error: {}", f, err).unwrap();
            }
        }
    }
    output.reset().unwrap();
}
