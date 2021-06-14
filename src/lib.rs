use std::fs;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_returns_config_object() {
        let args = vec![
            String::from("system arg :)"),
            String::from("query"),
            String::from("file.name")
        ];

        let conf = Config::new(&args)
            .expect("The structure should have been created, but it gave an error.");

        assert_eq!(conf.query.as_str(), "query");
        assert_eq!(conf.filename.as_str(), "file.name");
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn config_new_with_invalid_args_returns_error() {
        let args = vec![
            String::from("system arg :)"),
            String::from("query")
        ];

        Config::new(&args).unwrap();
    }

    #[test]
    fn run_returns_ok() {
        let args = vec![
            String::from("system arg :)"),
            String::from("query"),
            String::from("pushkin.txt")
        ];

        let conf = Config::new(&args).unwrap();

        assert_eq!(run(conf).unwrap(), ());
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn run_with_nonexistent_file_returns_err() {
        let args = vec![
            String::from("system arg :)"),
            String::from("query"),
            String::from("lermontov.odf")
        ];

        let conf = Config::new(&args).unwrap();
        run(conf).unwrap();
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text: {}", contents);
    Ok(())
}