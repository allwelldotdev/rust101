use std::{env, error::Error, fs};

// CLI argument structure ...
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let (query, file_path) = (&args[0], &args[1]);

        // Capture environment variable: MINIGREP_IGNORE_CASE
        // Check if env var is set or not
        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Read file content from filesystem ...
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read("poem.txt").expect("Should have been able to open the file"); // Read contents as Vec<u8>
    // let contents = String::from_utf8(contents).unwrap(); // Convert from Vec<u8> to String
    // dbg!(contents);

    let contents = fs::read_to_string(config.file_path)?;

    // let result = if config.ignore_case {
    //     search_case_insensitive(config.query, &contents)
    // } else {
    //     search(config.query, &contents)
    // };

    for line in search(config.query, &contents, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    // If case insensitive
    if ignore_case {
        let query = query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
    }
    // if case sensitive
    else {
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
    }

    results
}

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

// pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }
//     results
// }

// -- TESTS --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
