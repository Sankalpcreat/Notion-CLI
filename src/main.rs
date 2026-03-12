mod client;
mod commands;
mod credentials;

use std::process;

fn main() {
    let _ = dotenvy::dotenv();
    let cli = commands::build_cli();
    let matches = cli.get_matches();

    if let Err(e) = commands::run(&matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
