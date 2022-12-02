use std::env;
use std::fs;
use std::process;


struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Must provide filename");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn read_file() -> Result<String, &'static str> {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(config.filename)
        .expect("Error reading file");

    Ok(contents)
}
