use std::env;
use std::io;
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub struct Args {
    pub query : String,
    pub filename : String,
    pub case_sensitive: bool,
}

impl Args {
    pub fn new() -> Result<Args, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() != 3 {
            return Err("Expected 2 arguments");
        }

        Ok(Args { query : args[1].clone(),
            filename : args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
    }
}

pub fn run(args: Args) -> Result<(), Box<Error>> {
    for line in search(&args.query, &file_to_lines(&args.filename)?, args.case_sensitive) {
        println!("{}", line);
    }
    Ok(())
}

fn file_to_lines(file: &str) -> Result<Vec<String>, io::Error> {
    let mut f = File::open(file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s.split("\n").map(|l| String::from(l)).collect())
}

fn search<'a, 'b>(query: &'a str, lines: &'b Vec<String>, case_sensitive: bool) -> Vec<&'b str> {
    let mut pattern = String::from(query);

    if !case_sensitive {
        pattern = pattern.to_lowercase();
    }

    lines.iter().map(|l| &l[..]).filter(|l| {
        if case_sensitive {
            l.contains(&pattern)
        }
        else {
            l.to_lowercase().contains(&pattern)
        }}).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let lines = vec!["I wish", "I had", "duck feet"].into_iter().map(|s| String::from(s)).collect();
        assert_eq!(search("duck", &lines, true), vec!["duck feet"]);
        assert!(search("DUCK", &lines, true).is_empty());
    }

    #[test]
    fn case_insensitive() {
        let lines = vec!["I wish", "I had", "duck feet"].into_iter().map(|s| String::from(s)).collect();
        assert_eq!(search("duck", &lines, false), vec!["duck feet"]);
        assert_eq!(search("DUCK", &lines, false), vec!["duck feet"]);
    }
}
