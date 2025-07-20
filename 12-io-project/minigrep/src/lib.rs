use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip first argument
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let query = if case_sensitive {
        query
    } else {
        &query.to_lowercase()
    };

    contents
        .lines()
        .filter(|line| {
            let line = if case_sensitive {
                *line
            } else {
                &line.to_lowercase()
            };
            line.contains(query)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_results() {
        let query = "body";
        let contents = "\
Somebody you are
Give me that
Nobody you are not";

        assert_eq!(
            vec!["Somebody you are", "Nobody you are not"],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "give";
        let contents = "\
Somebody you are
Give me that
Nobody you are not";

        assert_eq!(Vec::<&str>::new(), search(query, contents, true));
    }

    #[test]
    fn case_insensitive() {
        let query = "give";
        let contents = "\
Somebody you are
Give me that
Nobody you are not";

        assert_eq!(vec!["Give me that"], search(query, contents, false),);
    }

    #[test]
    fn empty_input() {
        let query = "body";
        let contents = "";

        assert_eq!(Vec::<&str>::new(), search(query, contents, true));
    }
}
