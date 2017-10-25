extern crate clap;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
//extern crate serde_json;

use clap::App;
use std::str;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
//use serde_json::Value;

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Pulls data from Poloniex exchange.")
        .author("John B. <johnboydiv@gmail.com>")
        .args_from_usage("-n, --name=[NAME] 'Name of person to be greeted.'")
        .get_matches();

    let name = cli_args.value_of("name").unwrap_or("world");

    // Setup Tokio core to handle asynchronous IO
    let mut core = Core::new().unwrap();

    // Create HTTP client and register with Tokio core
    let client = Client::new(&core.handle());

    let uri = "http://httpbin.org/ip".parse().unwrap();
    let get = client.get(uri).map(|res| {
        println!("Response: {}", res.status());

        res.body().concat2();
    });

    let got = core.run(get).unwrap();
    println!("GET: {}", str::from_utf8(&got)?);
/*
    let client = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));

    let mut res = client
        .get("https://poloniex.com/public?command=return24hVolume")
        .header(Connection::close())
        .send()
        .unwrap();

    let mut msg = String::new();
    res.read_to_string(&mut msg).unwrap();

    let v: Value = serde_json::from_str(&msg.to_string()).unwrap();

    println!("BTC volume: {}", v["BTC_ETH"]["BTC"]);
    println!("ETH volume: {}", v["BTC_ETH"]["ETH"]);
    */
}
