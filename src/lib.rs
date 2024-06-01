use std::io;

use console::Term;
use crossterm::{
    cursor::MoveUp,
    terminal::{Clear, ClearType},
};

trait Input {
    fn ui(&self) -> String;
    fn render(&self);
    fn init(&self);
    fn fetch(&mut self) -> Result<String, io::Error>;
}

struct TextInput {
    label: String,
    input: String,
    is_error: bool,
}

impl TextInput {
    fn new(label: &str) -> TextInput {
        TextInput {
            label: String::from(label),
            input: String::new(),
            is_error: false,
        }
    }
}

impl Input for TextInput {
    fn ui(&self) -> String {
        let input_prompt = String::from(format!("-> {}|", self.input));
        format!("{input_prompt}")
    }

    fn init(&self) {
        let label = String::from(format!("$: {}", self.label));
        let input_prompt = String::from(format!("-> {}|", self.input));
        println!("{label}");
        println!("{input_prompt}");
    }

    fn render(&self) {
        let data = self.ui();
        replace_n_lines_with(1, data);
    }

    fn fetch(&mut self) -> Result<String, io::Error> {
        let stdout = Term::buffered_stdout();

        self.init();

        loop {
            self.render();
            let key_event = stdout.read_key()?;
            match key_event {
                console::Key::Char(charater) => {
                    self.input = format!("{}{charater}", self.input);
                }
                console::Key::Enter => {
                    self.is_error = false;
                    break;
                }
                console::Key::Backspace => {
                    if !self.input.is_empty() {
                        self.input.pop();
                    }
                }
                _ => {
                    self.is_error = true;
                    break;
                }
            };
        }

        if self.is_error {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))
        } else {
            Ok(self.input.clone())
        }
    }
}

pub fn text_input(l: &str) -> Result<String, io::Error> {
    let mut input = TextInput::new(l);
    input.fetch()
}


pub fn replace_n_lines_with(n: u16, replace_str: String) {
    print!("{}", MoveUp(n));
    print!("{}", Clear(ClearType::CurrentLine));
    println!("{replace_str}");
}
