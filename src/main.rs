extern crate console;

use std::{env, fs};
use std::str::FromStr;
use std::path::PathBuf;

use structopt::StructOpt;

use console::{Style, StyledObject};


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

fn exclude_hidden_files(files: Vec<String>, exclude: bool) -> Vec<String>{
    if exclude{
        files.drain_filter(|f| f.starts_with(".")).collect()::<Vec<String>>;
    }

    println!("{:?}", files);

    files
}

fn color_output(set_color: Colors, str_output: String, color: Style) -> StyledObject<String> {
    let default_style = Style::new().white();

    let colored_output = match set_color {
        Colors::Never => default_style.apply_to(str_output),
        _ => color.apply_to(str_output)
    };

    colored_output
}


fn list_dir(dir: Option<PathBuf>, output_color: Colors, show_all: bool) -> std::io::Result<()>{
    let mut childs_files: Vec<String> = Vec::new();
    let mut childs_directories: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir.unwrap())? {
        let filename = entry?.file_name().into_string().unwrap();

        if fs::metadata(&filename)?.is_dir(){
            childs_directories.push(filename);
        }else{
            childs_files.push(filename)
        }
    }

    let dir_style = Style::new().blue().bold();

    let joinf: String = exclude_hidden_files(childs_files, show_all).join(" ");
    let joind: String = exclude_hidden_files(childs_directories, show_all).join(" "); 
    let joind: StyledObject = color_output(output_color, joind, dir_style);

    println!("{} {}", joinf, joind);

    Ok(())
}

fn main() {
    let args = Ls::from_args();
    println!("{:?}", args);

    let cdir = match args.directory {
        Some(i) => Some(i),
        None => Some(env::current_dir().unwrap())
    };

    list_dir(cdir, args.color, args.all);
}
