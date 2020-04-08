#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate console;
extern crate structopt;

use std::env;
use std::io::Write;
use std::str::FromStr;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use console::*;

const IMAGE_TYPES: [&str; 3] = ["png", "jpg", "jpge"];
const COMPRESSION_TYPES: [&str; 3] = ["tar.gz", "zip", "xz"];

#[derive(Debug)]
enum Colors {
    Always,
    Never,
    Auto
}


#[derive(StructOpt, Debug)]
struct Rs{
    #[structopt(parse(from_os_str))]
    directory: Option<PathBuf>,

    #[structopt(short, long)]
    all:bool,

    #[structopt(short="l", long="author")]
    author: bool,

    #[structopt(short="C", long="color", default_value="always")]
    color: Colors,
}


#[derive(Debug)]
struct FileDs {
    name: String,
    extension: Option<String>,
    metadata: std::fs::Metadata,
    formated_name: Option<String>,
}

impl FileDs {
    fn format(&self) {
        if self.metadata.is_dir() {
            //self.formated_name: console::StyledObject = style(self.name).blue().bold();
            println!("{:?}", style(&self.name).blue().bold());
        }
        if self.metadata.is_file() {
            match &self.extension {
                Some(e) => {
                    println!("{:?}", e);
                },
                _ => {} 
            }
        }
    }
}

impl FromStr for Colors {
    type Err = std::string::ParseError;

    fn from_str(color: &str) -> Result<Self, Self::Err>{
        match color {
            "never" => Ok(Colors::Never),
            "always" => Ok(Colors::Always),
            _ => Ok(Colors::Auto),
        }
    }
}


impl Rs {
    fn manager(&mut self) {
        match &self.directory {
            None => {
                self.directory = Some(env::current_dir().unwrap());
                self.list_dir();
            },
            Some(i) => {
                self.list_dir();
            }
        };
    }

    fn list_dir(&self) {
        let mut files: Vec<FileDs> = Vec::new();

        let dir_ref = self.directory.as_ref().unwrap();

        for entry in dir_ref.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();
                let name = entry.file_name().into_string().unwrap();

                // let extension = Path::new(&name)
                // 	.extension()
                // 	.and_then(std::ffi::OsStr::to_str);

                files.push(FileDs {name, metadata, formated_name: None, extension: None});
            }
        }

        for entry in files.iter() {
			entry.format();
        }
    }

}


fn main() {
     Rs::from_args().manager();
}
