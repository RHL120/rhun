mod config;
use std::process::exit;

fn main() {
    let username = runas::get_username().unwrap_or_else(|| {
        eprintln!("Failed to get username");
        exit(1);
    });
    let cfg = config::get_config(&username).unwrap_or_else(|| {
        eprintln!("runas is not configured for {}.", username);
        exit(1);
    });
}
