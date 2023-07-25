use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?; //? "?"他發生錯誤時不會發生panic!而是會把錯誤返回給調用者
    let results = if config.case_sensitive {
        search_case_insensitive(&config.search, &contents)
    } else {
        search(&config.search, &contents)
    };
    for lines in results {
        println!("{}", lines);
    }
    return Ok(());
}

pub struct Config {
    pub search: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments");
        }
        // let search = args.get(1).unwrap();
        // let file_name = args.get(2).unwrap();
        args.next();

        let search = match args.next() {
            Some(value) => value,
            None => return Err("no such string"),
        };
        let file_name = match args.next() {
            Some(value) => value,
            None => return Err("no such file_name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_ok();
        return Ok(Config {
            search: search,
            file_name: file_name,
            case_sensitive,
        });
        // return Ok(Config {
        //     search: search.to_owned(),
        //     file_name: file_name.to_owned(),
        //     case_sensitive,
        // });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_from_content() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, production.
Pick three.";

        assert_eq!(vec!["safe, fast, production."], search(query, contents))
    }

    #[test]
    fn test_insensitive_search_from_content() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, production.
Pick three.
Duct";
        assert_eq!(
            vec!["safe, fast, production.", "Duct"],
            search_case_insensitive(query, contents)
        )
    }
}
