use std::{fs, error::Error};

pub struct Config {
    pub filename: String,
    pub target: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let target = args[2].clone();
        Ok(
            Config {
            filename: filename,
            target: target
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let content = fs::read_to_string(&self.filename)?;

        Ok(())
    }
}
