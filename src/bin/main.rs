use std::error::Error;
use std::process;

use modin_rs::text_input;

/*
    let stdout = Term::buffered_stdout();
    let mut s = 1;

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'f' => {
                    s += 1;
                    clear_n_lines(1, s.to_string())
                }
                _ => break 'game_loop,
            }
        }
    }

    Ok(())
* */

fn run() -> Result<(), Box<dyn Error>> {
    let email = text_input("Enter your email")?;

    println!("User email is {email}");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Application error: {:?}", err);
        process::exit(1)
    }
}
