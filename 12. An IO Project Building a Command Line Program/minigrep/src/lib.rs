use std::fs;
use std::env;
use std::error::Error;

pub fn run ( config: Config ) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive 
    {
        search( &config.query, &contents, true )
    }
    else 
    {
        search( &config.query, &contents, false)
    };

    for line in results
    {
        println!( "{}", line );
    }

    Ok(())
}

pub fn search <'a> ( query: &str, contents: &'a str, sensitive: bool ) -> Vec<&'a str> 
{
    let lines = contents.lines();

    if sensitive {
        return lines.filter(|line| line.contains(query)).collect()
    }
    else {
        return lines.filter(|line| line.to_lowercase().contains(query)).collect()
    }

}

pub struct Config 
{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config 
{
    pub fn new (mut args: env::Args) -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = match args.next() {
            Some(arg) => arg, 
            None => return Err("Didn't get a query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn one_result() 
    {
        let query = "duct";
        let contents = "Rust:
        \nsafe, fast, productive.
        \nPick three.";

        assert_eq!( vec![ "safe, fast, productive." ], search( query, contents ) );

    }

    #[test]
    fn case_insensitive ()
    {
        let query = "rUst";
        let contents = "Rust:
        \nsafe, fast, productive.
        \nPick three.
        \nTrust me.";

        assert_eq!(
            vec![ "Rust:", "Trust me."],
            search_case_insensitive( query, contents )
        );
    }
}


