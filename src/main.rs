#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate console;
extern crate structopt;

use std::env;
use std::io::Write;
use std::ffi::OsStr;
use std::str::FromStr;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::fs::Metadata;

use structopt::StructOpt;

use console::*;

const MAX_COLUMN_SIZE: usize = 6;
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
    metadata: Metadata,
    formated_name: Option<StyledObject<String>>,
}

impl FileDs {
    /// Define the color output of the files
    fn set_color(ext: &str, filename: &String) -> StyledObject<String> {
        for file_type in PINK_OUTPUT_FILES.iter() {
            if ext == *file_type {
                return style(filename.clone()).magenta().bold();
            }
        }

        for file_type in RED_OUTPUT_FILES.iter() {
            if ext == *file_type {
                return style(filename.clone()).red().bold();
            }
        }
        style(filename.clone()).white()	
    }

    /// Initialize the struct
    fn new(name: String, metadata: std::fs::Metadata) -> Self {
        let extension = Path::new(&name)
            .extension()
            .and_then(OsStr::to_str);

        let formated_name = match extension {
            Some(e) => Some(Self::set_color(&extension.unwrap(), &name)),
            _ => {
                if metadata.is_dir() {
                    Some(style(name.clone()).blue().bold())
                } else {
                    Some(style(name.clone()).white())
                }
            }
        };

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


fn sort(vec: &Vec<FileDs>) -> Vec<FileDs> {
    let mut sorted_vec = Vec::new();
    let mut index_vec = 0;
    let mut index_vec2 = 1;
    loop {
        if vec[index_vec].name.to_lowercase() > vec[index_vec2].name.to_lowercase() {
            sorted_vec.push(vec[index_vec2]);
        } else {
            sorted_vec.push(vec[index_vec]);
        }
        
        index_vec += 1;
        index_vec2 += 1;

        if index_vec >= vec.len() {
            break;
        }
    }
    
    sorted_vec
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
                if name.starts_with(".") && !self.all{
                    continue
                }
                files.push(FileDs::new(name, metadata));
            }
        }

        println!("{:?}", sort(&files));

		//files.sort_by(|a, b| a.name.cmp(&b.name));

    //     let terminal = Term::stdout();
    //     let mut output = String::new();
    //     let mut chunks = files.chunks(MAX_COLUMN_SIZE);

    //     while let chunk = chunks.next().unwrap() {
    //         for row in chunk {
    //             output.push_str(&row.name);
    //             output.push_str(" ");
    //         }
            
    //         terminal.write_line(&output).unwrap();
    //         output = String::new();
    //     };

    // }
}


fn main() {
     Rs::from_args().manager();
}
