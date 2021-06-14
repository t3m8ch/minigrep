use std::{fs, env};
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

    #[test]
    fn search_returns_line_which_contains_query() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_returns_line_which_contains_query_unicode() {
        let query = "Привет, мир! ⛵";
        let contents = "\
Это мой текст.
Hello, world! Привет, мир! ⛵⛵⛵";
        assert_eq!(vec!["Hello, world! Привет, мир! ⛵⛵⛵"], search(query, contents));
    }

    #[test]
    fn search_returns_several_lines_which_contain_query() {
        let query = "Hello";
        let contents = "\
Hi, people!
Hello, world!
Hello, country!
I'm Artem.";
        assert_eq!(
            vec!["Hello, world!", "Hello, country!"],
            search(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive_returns_line_which_contains_query() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }

    #[test]
    fn search_case_insensitive_returns_line_which_contains_query_regardless_of_case() {
        let query = "THReE";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Pick three."], search_case_insensitive(query, contents));
    }

    #[test]
    fn search_case_insensitive_returns_line_which_contains_query_unicode() {
        let query = "привет, МИР! ⛵";
        let contents = "\
Это мой текст.
Hello, world! Привет, мир! ⛵⛵⛵";
        assert_eq!(
            vec!["Hello, world! Привет, мир! ⛵⛵⛵"],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query.as_str()) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
