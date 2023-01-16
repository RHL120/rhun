mod config;
use std::env;
use std::process::Command;

fn main() {
    println!("pass: {:#?}", runas::read_password("[runas] password: "));
}
