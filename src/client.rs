extern crate ring;

use self::ring::aead;

use super::parse_config;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

pub fn launch(config_path: &str) -> Result<(), Box<Error>>
{
    let mut f = File::open(config_path)?;
    let config = Config::new(&mut f)?;

    let mut key_file = File::open("key.txt")?;
    let mut key = String::new();
    key_file.read_to_string(&mut key)?;
    let key = key.trim();

    let algorithm = &aead::AES_256_GCM;
    println!("{} {}", algorithm.key_len(), key.len());
    let opener = aead::OpeningKey::new(algorithm, key.as_bytes())?;
    println!("{:?}", opener.algorithm());
    match TcpStream::connect(*config.socket_addr())
    {
        Ok(mut stream) =>
        {
            println!("worked");
            let buf = &mut [0; 64];
            stream.write(buf).unwrap();
        }
        Err(err) =>
        {
            println!("didn't work, {:?}", err);
        }
    }
    Ok(())
}

struct Config
{
    socket_addr: SocketAddr,
}

impl Config
{
    pub fn new(f: &mut File) -> Result<Config, Box<Error>>
    {
        let mut config_string = String::new();
        f.read_to_string(&mut config_string)?;
        let ip: String = parse_config(config_string.lines(), "ip")?;
        let port: String = parse_config(config_string.lines(), "port")?;
        let socket_addr = format!("{}:{}", ip, port).parse()?;
        Ok(Config { socket_addr })
    }
    pub fn socket_addr(&self) -> &SocketAddr
    {
        &self.socket_addr
    }
}
