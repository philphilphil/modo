use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use clap::Parser;

/// Query todos in markdown files.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the root folder of the search
    #[clap(short, long, parse(try_from_os_str=parse_path), value_name = "PATH")]
    path: Option<PathBuf>,

    /// Query for the todos
    #[clap(short, long, default_value = "", parse(from_str))]
    query: String,
}

fn main() {
    let args = Args::parse();
    modo::modo(args.path.unwrap(), args.query);
}

fn parse_path(str: &OsStr) -> Result<PathBuf, String> {
    let path = Path::new(&str);

    if !path.exists() {
        return Err(String::from("Path does not exist."));
    } else if !path.is_dir() {
        return Err(String::from("Path is not a directory."));
    }

    Ok(path.to_path_buf())
}
