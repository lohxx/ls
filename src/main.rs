extern crate console;
extern crate structopt;

use std::io::Write;
use std::env;
use std::str::FromStr;
use std::path::PathBuf;

use structopt::StructOpt;

use console::*;


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

    fn exclude_hidden_files(&self, directory_entries: Vec<String>) -> Vec<String> {
        let mut updated_dir = Vec::new();

        if !self.all {
            for (index, entry) in directory_entries.iter().enumerate() {
                if entry.starts_with('.') {
                    continue
                }
                updated_dir.push(entry.to_string());
            }

            return updated_dir
        }

        directory_entries

    }

    fn list_dir(&self) {
        let mut files: Vec<String> = Vec::new();
        let mut directories: Vec<String> = Vec::new();
        let dir_ref = self.directory.as_ref().unwrap();

        for entry in dir_ref.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();

                if metadata.is_dir() {
                    directories.push(
                        entry.file_name().into_string().unwrap()
                    );
                }
                if metadata.is_file() {
                    files.push(
                        entry.file_name().into_string().unwrap()
                    );
                }
            }
        }

        let files = self.exclude_hidden_files(files);
        let directories = self.exclude_hidden_files(directories);

        println!("{:?}", files);
        println!("{:?}", directories);
    }

}


fn main() {
    let mut command = Rs::from_args();    
    command.manager();
}
