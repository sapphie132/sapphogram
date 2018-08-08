extern crate sapphogram;

use sapphogram::server;
fn main()
{
    match server::launch("server_config.txt")
    {
        Err(err) => eprintln!("{}", err),
        _ => (),
    };
}
