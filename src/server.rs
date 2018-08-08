extern crate mio;
extern crate ring;

use self::mio::net::{TcpListener, TcpStream};
use std;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

use self::ring::aead;
use super::parse_config;

pub fn launch(config_path: &str) -> Result<(), Box<Error>>
{
    //set up config
    let mut config = File::open(config_path)?;
    let config = Config::new(&mut config)?;
    //set up the tcp listener
    let listener = TcpListener::bind(&config.socket_addr)?;
    let mut key_file = File::open("key.txt")?;
    let mut key = String::new();
    key_file.read_to_string(&mut key)?;
    let key = key.trim();

    //encryption
    let algorithm = &aead::AES_256_GCM;
    println!("{} {}", algorithm.key_len(), key.len());
    let _opener = aead::OpeningKey::new(algorithm, key.as_bytes())?;
    let _closer = aead::SealingKey::new(algorithm, key.as_bytes())?;

    //active streams
    let mut streams = std::collections::HashMap::new();
    loop
    {
        if let Ok((stream, _)) = listener.accept()
        {
            println!("New client");
            let len = streams.len();
            stream.set_keepalive(Some(config.keep_alive_duration))?;
            streams.insert(len, stream);
        }

        handle_streams(&mut streams);
        //handle individual streams
        //sleep
        thread::sleep(config.loop_duration);
        println!("{}", streams.len());
    }
}

fn handle_streams(streams: &mut std::collections::HashMap<usize, TcpStream>)
{
    let mut buffer: &mut [u8] = &mut [0; 1024];
    let mut to_remove = Vec::new();
    {
        for (num, ref mut stream) in streams.iter()
        {
            //check if there was a new message
            match stream.read(&mut buffer)
            {
                Ok(num_read) =>
                {
                    println!("Message length: {}", num_read);

                    //send keepalive manually if we didn't receive a response
                    if num_read == 0
                    {
                        match stream.write(b"i")
                        {
                            Ok(_) => println!("hi"),
                            Err(err) =>
                            {
                                match err.kind()
                                {
                                    std::io::ErrorKind::ConnectionAborted => to_remove.push(*num),
                                    _ => panic!(),
                                };
                            }
                        }
                    }
                }
                Err(err) => println!("here {:?}", err),
            }
        }
    }

    for num in to_remove.iter()
    {
        streams.remove(num);
    }
}

struct Config
{
    loop_duration: Duration,
    keep_alive_duration: Duration,
    socket_addr: SocketAddr,
}
impl Config
{
    pub fn new(cfg_file: &mut File) -> Result<Config, Box<std::error::Error>>
    {
        //get the whole config file as text
        let mut file = String::new();
        cfg_file.read_to_string(&mut file)?;

        //loop duration
        let s = parse_config(file.lines(), "loop_duration_s")?;
        let ns = parse_config(file.lines(), "loop_duration_ns")?;
        let loop_duration = Duration::new(s, ns);

        //socket
        let ip: String = parse_config(file.lines(), "ip")?;
        let port: String = parse_config(file.lines(), "port")?;
        let socket_addr = format!("{}:{}", ip, port).parse()?;

        //keep alive duration
        let s = parse_config(file.lines(), "keep_alive_duration_s")?;
        let ns = parse_config(file.lines(), "keep_alive_duration_ns")?;
        let keep_alive_duration = Duration::new(s, ns);

        Ok(Config {
            loop_duration,
            socket_addr,
            keep_alive_duration,
        })
    }
}
