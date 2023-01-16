mod config;

fn main() {
    let username = runas::get_username().unwrap();
    let pass = runas::read_password(&format!("[runas] password for {}: ", username)).unwrap();
    println!("{:#?}", runas::check_password(&username, &pass));
}
