use clap::Parser;
use modo::todo::Todo;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
mod ui;

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

    if let Err(e) = ui::draw_ui(&args.query, args.path.as_ref().unwrap()) {
        println!("Error: {}", e);
    }
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
