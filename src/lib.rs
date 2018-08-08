use std::error::Error;
use std::fmt;

pub mod client;
pub mod server;

fn parse_config<T>(config_file_lines: std::str::Lines, option_name: &str) -> Result<T, Box<Error>>
where
    T: std::str::FromStr, //needs to be able to convert to a string
    <T as std::str::FromStr>::Err: 'static + std::error::Error, //needs to be able to throw Errors that implement Error
{
    let s: T = config_file_lines.filter(|string| string.contains(option_name))
        .next() //get the next 
        .ok_or(format!(r#"Malformed config file, lacking a line for "{}""#, option_name))?
        .rsplit(':')
        .next()
        .ok_or(format!(r#"Malformed config file, unable to find argument for "{}""#, option_name))?
        .trim()
        .parse()?;
    Ok(s)
}

#[derive(Debug)]
struct StringErr
{
    message: Box<String>,
}

impl fmt::Display for StringErr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.message)
    }
}

impl Error for StringErr
{
    fn description(&self) -> &str
    {
        &self.message[..]
    }
}

impl std::convert::From<std::string::String> for StringErr
{
    fn from(string: String) -> StringErr
    {
        StringErr {
            message: Box::new(string),
        }
    }
}
