use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough args");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone()
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    };
    matches
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let matches = search(&config.query, &contents);
    for item in matches {
        println!("{}", item);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick one";

        assert_eq!(search(query,contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn two_results() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick one\nin ductapero";

        assert_eq!(search(query,contents), vec!["safe, fast, productive.", "in ductapero"]);
    }
}


