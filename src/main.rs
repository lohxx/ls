extern crate console;

use std::{env, fs};
use std::str::FromStr;
use std::path::PathBuf;

use structopt::StructOpt;

use console::{Term, style};


#[derive(Debug)]
enum Colors {
    Always,
    Never,
    Auto
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


#[derive(StructOpt, Debug)]
struct Ls{
    #[structopt(parse(from_os_str))]
    directory: Option<PathBuf>,

    #[structopt(short, long)]
    all:bool,

    #[structopt(short="l", long="author")]
    author: bool,

    #[structopt(short="C", long="color", default_value="always")]
    color: Colors,
}


fn list_dir(dir: Option<PathBuf>) -> std::io::Result<()>{
    let mut childs_files: Vec<String> = vec![];
    let mut childs_directories: Vec<String> = vec![];

    for entry in fs::read_dir(dir.unwrap())? {
        let filename = entry?.file_name().into_string().unwrap();
        if !filename.starts_with("."){
            if fs::metadata(&filename)?.is_dir(){
                childs_directories.push(filename);
            }else{
                childs_files.push(filename)
            }

        }
    }

    let term = Term::stdout();
    let joind: String = childs_directories.join(" ");
    let joinf: String = childs_files.join(" ");
    println!("{} {}", joinf, style(joind).blue().bold());

    Ok(())
}

fn main() {
    let args = Ls::from_args();
    println!("{:?}", args);

    let cdir = match args.directory {
        Some(i) => Some(i),
        None => Some(env::current_dir().unwrap())
    };

    list_dir(cdir);
}
