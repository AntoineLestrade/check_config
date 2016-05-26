extern crate regex;
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs;
use std::io::Error;

pub fn list_git_files(path: &Path, re: &regex::Regex) -> Result<Vec<String>, Error> {
    let mut result: Vec<String> = Vec::<String>::new();

    let output = try!(Command::new("git")
        .current_dir(path)
        .arg("ls-files")
        .arg("--full-names")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output());
    let raw_list = match String::from_utf8(output.stdout) {
        Ok(l) => l,
        Err(e) => {
            panic!("{}", e);
        }
    };

    for p in raw_list.lines().filter(|l| re.is_match(l)) {
        result.push(String::from(p));
    }
    return Ok(result);
}


pub fn list_files(path: &Path, re: &regex::Regex) -> Result<Vec<String>, Error> {
    let mut result: Vec<String> = Vec::<String>::new();

    let metadata = try!(fs::metadata(path));
    if metadata.is_file() {
        if let Some(file_path) = path.to_str() {
            if re.is_match(file_path) {
                result.push(String::from(file_path));
            }
        }
    } else if metadata.is_dir() {
        for entry in try!(fs::read_dir(path)) {
            let e = try!(entry);
            let list = try!(list_files(e.path().as_path(), re));
            for f in list {
                result.push(f);
            }
        }
    }
    Ok(result)
}
