use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("File name not specified");
        } else if args.len() > 2 {
            return Err("Too many arguments");
        }
        let filename = args[1].clone();
        Ok ( Config {filename})
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    write_file(split_lines(&content)).unwrap();
    Ok(())
}

pub fn split_lines (content: &String) -> Vec<String>{
     content.rsplit("\r\n").map(str::to_string).collect()
}

pub fn write_file (content: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create("text-reversed.txt")?;
    for line in content {
        file.write_all((line + "\n").as_ref())?;
    }
    Ok(())
}

