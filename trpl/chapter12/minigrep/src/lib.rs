use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// 失败返回一个实现 Error 类的 Trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contens = fs::read_to_string(config.filename)?;
    println!("With text: {}\n", contens);
    Ok(())
}