use colored::Colorize;
use std::io;

use console::Term;
use crossterm::{
    cursor::MoveUp,
    terminal::{Clear, ClearType},
};

trait Input {
    fn ui(&self) -> String;
    fn render_change(&self);
    fn init(&self);
    fn fetch(&mut self) -> Result<String, io::Error>;
}

struct TextInput {
    nerd: bool,
    label: String,
    input: String,
    is_error: bool,
}

impl TextInput {
    fn new(label: &str, nerd: bool) -> TextInput {
        TextInput {
            label: String::from(label),
            input: String::new(),
            is_error: false,
            nerd,
        }
    }
}

impl Input for TextInput {
    fn ui(&self) -> String {
        let input_prompt = if self.nerd {
            String::from(format!("{} {}|", "❯".green(), self.input))
        } else {
            String::from(format!("{} {}|", "->".green(), self.input))
        };

        format!("{input_prompt}")
    }

    fn init(&self) {
        let label = if self.nerd {
            String::from(format!(" {}", self.label))
        } else {
            String::from(format!("$ {}", self.label))
        };
        println!("{}", label.blue());
        let input_prompt = self.ui();
        println!("{input_prompt}");
    }

    fn render_change(&self) {
        let data = self.ui();
        replace_n_lines_with(1, data);
    }

    fn fetch(&mut self) -> Result<String, io::Error> {
        let stdout = Term::buffered_stdout();

        self.init();

        loop {
            self.render_change();
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

// struct CheckBox(bool, String);
//
// struct CheckBoxInput {
//     boxs: Vec<CheckBox>,
// }

pub struct Modin {
    nerd: bool,
}

impl Modin {
    pub fn new() -> Modin {
        Modin { nerd: false }
    }

    pub fn use_nerd(&mut self) {
        self.nerd = true;
    }

    pub fn text_input(&self, l: &str) -> Result<String, io::Error> {
        let mut input = TextInput::new(l, self.nerd);
        input.fetch()
    }
}

pub fn replace_n_lines_with(n: u16, replace_str: String) {
    print!("{}", MoveUp(n));
    print!("{}", Clear(ClearType::CurrentLine));
    println!("{replace_str}");
}
