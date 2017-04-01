extern crate clap;

use clap::App;

fn main() {
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("John B. <johnboydiv@gmail.com>")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .args_from_usage("-n, --name=[NAME] 'Name of person to be greeted.'")
        .get_matches();

    let name = cli_args.value_of("name").unwrap_or("world");

    println!("Hello, {}!", name);
}
