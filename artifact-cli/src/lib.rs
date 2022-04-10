use std::env;
use std::error::Error;
use std::fs;

// use json_minimal::*;
use serde::Deserialize;
use serde_json::json;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);
    // let results = if config.case_sensitive {
    //     search(&config.query, &contents)
    // } else {
    //     search_case_insensitive(&config.query, &contents)
    // };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();

    let json: serde_json::Value =
    serde_json::from_str(contents).expect("JSON was not well-formatted");
    let packages = json.get("devDependencies").unwrap().as_object().unwrap();
    packages.into_iter().for_each(|(key, value)| {
        match key.starts_with("sveltejs") {
            true => {
                results.push(key.clone());
            },
            false => {
                // println!("{}", key);
            },
        }

    });

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    results
}

// fn extractJson(json: &Json) -> Vec<Json> {
//     match json {
//         Json::STRING(value) => {
//             panic!("The value is unsupported; Type is STRING. {:?}", value);
//         },
//         Json::OBJECT { name, value } => {
//             // panic!("The value is unsupported; Type is OBJECT. {:?}: {:?}", name, value);
//             extractJson(value);
//         },
//         Json::ARRAY(value) => {
//             panic!("The value is unsupported; Type is ARRAY. {:?}", value);
//         },
//         Json::JSON(value) => {
//             panic!("The value is unsupported; Type is JSON. {:?}", value);
//         },
//         _ => {
//             panic!("The value is unsupported; Type is unsupported.");
//         }
//     };
// }

// pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a String> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }

//     results
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
}
