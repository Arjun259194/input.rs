use std::error::Error;
use std::process;

use modin_rs::Modin;

fn run() -> Result<(), Box<dyn Error>> {
    let modin = Modin::new();

    let email = modin.text_input("Enter your email")?;
    let username = modin.text_input("What is your name?")?;

    println!("User email is {email}");
    println!("Your name is {username}");

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Application error: {:?}", err);
        process::exit(1)
    }
}
