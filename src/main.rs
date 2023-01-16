mod config;

#[derive(Debug)]
enum Error {

}

fn main() -> Result<(), Error>{
    let username = runas::get_username();
    Ok(())
}
