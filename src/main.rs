extern crate clap;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde_json;

use std::io::Read;
use clap::App;
use hyper::Client;
use hyper::header::Connection;
use hyper::net::HttpsConnector;
use serde_json::Value;

fn main() {
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("John B. <johnboydiv@gmail.com>")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .args_from_usage("-n, --name=[NAME] 'Name of person to be greeted.'")
        .get_matches();

    let name = cli_args.value_of("name").unwrap_or("world");

    println!("Hello, {}!", name);

    let client = Client::with_connector(HttpsConnector
        ::new(hyper_rustls::TlsClient::new()));

    let mut res = client.get("https://poloniex.com/public?command=return24hVolume")
            .header(Connection::close())
            .send()
            .unwrap();

    let mut msg = String::new();
    res.read_to_string(&mut msg).unwrap();

    let v: Value = serde_json::from_str(&msg.to_string()).unwrap();

    println!("BTC volume: {}", v["BTC_ETH"]["BTC"]);
    println!("ETH volume: {}", v["BTC_ETH"]["ETH"]);
}
