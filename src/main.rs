#[macro_use] extern crate lazy_static;

mod ls;
mod parse_file;

extern crate ansi_term;
extern crate getopts;
extern crate regex;

use ansi_term::Colour::{Red,Green,Yellow};
use getopts::Options;

use std::env;
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
        Ok (m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") || matches.free.len() != 1 {
        print_usage(&program, opts);
        return;
    }

    let path = Path::new(&matches.free[0]);
    let files = if matches.opt_present("g") {
    		ls::list_files(&path, &regex::Regex::new(r".*\.config$").unwrap())
    	}
    	else {
    		ls::list_git_files(&path, &regex::Regex::new(r".*\.config$").unwrap())
    	};

    for f in files.unwrap() {
        match parse_file::parse_file(&f, parse_file::Parser::DotNet) {
            Ok (list) => {
                for item in list {
                    let sentence =
                             format!("File: {}; Name: {}; Server: {}; DB: {}",
                                                 f,
                                                 item.cs_name,
                                                 item.server_name,
                                                 item.db_name);
                    if item.is_good {
                        println!("{}", Green.paint(sentence));
                    }
                    else {
                        println!("{}", Red.paint(sentence));
                    }
                }
            }
            Err (err) => {
                println!("{}",
                         Yellow.paint(format!("File: {}; Error: {}",
                                           f,
                                           err)));
            }
        }
    }
}

