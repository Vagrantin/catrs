use clap::{App, Arg};
use std::error::Error;
use std::fs::{File,read_to_string};
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("Catrs")
        .version("0.1.0")
        .author("Matth DucDuc <md@example.com>")
        .about("Rust Cat")
        .arg(
            Arg::with_name("file")
            .value_name("FILE")
            .help("Input file(s)")
            .required(true)
            .multiple(true)
            .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number the lines")
            .takes_value(false)
            .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::with_name("number-nonblank")
            .short("b")
            .long("number-nonblank")
            .help("Number nonblank lines")
            .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_lines(filename: &str) -> Vec<(usize,String)> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(num, line)| (num +1 , String::from(line)))
        .collect()
}

fn read_lines_n(filename:&String) -> () {
        if &filename != &"-" { 
         let lines = read_lines(&filename); 
          for (num, line) in lines{
              println!("{:>6}	{}", num, line);
          }
        } else {
         let stdin = io::stdin();
         let mut line_num = 0;
          for line in stdin.lock().lines(){
              line_num += 1;
              println!("{:>6}\t{}", line_num, line.unwrap());
          }
        }
}

fn read_lines_b(filename:&String) -> () {
        if &filename != &"-" { 
         let lines = read_lines(&filename); 
          for (num, line) in lines{
              println!("{}	{}", num, line);
          }
        } else {
         let stdin = io::stdin();
          for line in stdin.lock().lines(){
              println!("bb {}", line.unwrap());
          }
        }
}

fn read_lines_normal(filename:&String) -> () {
        if &filename != &"-" { 
         let lines = read_lines(&filename); 
          for (_num, line) in lines{
              println!("{}", line);
          }
        } else {
         let stdin = io::stdin();
          for line in stdin.lock().lines(){
              println!("{}", line.unwrap());
          }
        }
}
pub fn run (config: Config) -> MyResult<()> {
    //let filename = config.files;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => (),
        }
   if config.number_lines {
       read_lines_n(&filename)
   } else if config.number_nonblank_lines {
       read_lines_b(&filename)
   } else {
       read_lines_normal(&filename)
   }
  }
    Ok(())
}
