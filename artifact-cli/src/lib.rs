// use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;

use console::Term;

use console;
use dialoguer;
use indicatif;
const PACKAGE_NAME_QUERY: &str = "@sveltejs";

pub struct Config {
    pub query: String,
    pub filename: String,
    // pub case_sensitive: bool,
}

pub struct Package {
    name: String,
    version: String,
}

const QUERY: &str = "devDependencies";
const FILENAME: &str = "/home/biscuitech/repos/BiscuiT-svelte/package.json";

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // let query = args[1].clone();
        // let filename = args[2].clone();

        let query = QUERY.to_string();
        let filename = FILENAME.to_string();

        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            // case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);

    // for line in results {
    //     println!("{}:{}", line.name, line.version);
    // }

    let term = Term::stdout();
    term.write_line("Select version to upgrade");
    for line in results {
        term.write_line(&format!("{}:{}", line.name, line.version));
    }
    term.
    // thread::sleep(Duration::from_millis(2000));
    // term.clear_line();
    // term.write_vectored(results);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Package> {
    let mut results: Vec<Package> = Vec::new();

    let json: serde_json::Value =
    serde_json::from_str(contents).expect("JSON was not well-formatted");
    let packages = json.get("devDependencies").unwrap().as_object().unwrap();
    packages.into_iter().for_each(|(key, value)| {
        match key.starts_with(PACKAGE_NAME_QUERY) {
            true => {
                results.push(Package {
                    name: key.to_string(),
                    version: value.to_string(),
                });
            },
            false => {
                // println!("{}", key);
            },
        }

    });
    // println!("{:?}", packages);
    results
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
// }
