extern crate sapphogram;

use sapphogram::client;
fn main()
{
    client::launch("client_config.txt").unwrap();
}
