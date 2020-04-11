#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate console;
extern crate structopt;

use std::env;
use std::io::Write;
use std::ffi::OsStr;
use std::str::FromStr;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use console::*;

const PINK_OUTPUT_FILES: [&str; 3] = ["png", "jpg", "jpge"];
const RED_OUTPUT_FILES: [&str; 4] = ["tar.gz", "zip", "xz", "deb"];

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
    metadata: std::fs::Metadata,
    formated_name: Option<console::StyledObject<String>>,
}

impl FileDs {
    fn set_color(ext: &str, filename: &String) -> StyledObject<String> {
        for img_type in PINK_OUTPUT_FILES.iter() {
            if ext == *img_type {
                return style(filename.clone()).magenta().bold();
            }
        }

        for comp in RED_OUTPUT_FILES.iter() {
            if ext == *comp {
                return style(filename.clone()).red().bold();
            }
        }
        style(filename.clone()).white()	
    }

    fn new(name: String, metadata: std::fs::Metadata) -> Self {
        let mut formated_name = None;
        let extension = Path::new(&name)
            .extension()
            .and_then(OsStr::to_str);

        if metadata.is_dir() {
            formated_name = Some(style(name.clone()).blue().bold());
        }

        match extension {
            Some(e) => {
                formated_name = Some(Self::set_color(&extension.unwrap(), &name));	
            },
            _ => ()
        }

        Self {
            name,
            metadata,
            formated_name,
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

                files.push(FileDs::new(name, metadata));
            }
        }

        for entry in files {
        	println!("{:?}", entry.formated_name);
        }      
    }

}


fn main() {
     Rs::from_args().manager();
}
