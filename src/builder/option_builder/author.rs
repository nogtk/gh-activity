use super::GhOption;
use inquire::{Confirm, Text};

pub fn build() -> GhOption {
    let c = Confirm::new("Do you specify the author?")
        .with_default(false)
        .prompt();

    match c {
        Ok(true) => {
            let t = Text::new("Who is the author?")
                .with_default("@me")
                .with_help_message("type @me to search for your own repos")
                .prompt()
                .unwrap();
            GhOption {
                arg: Some(String::from("--author")),
                content: Some(t),
            }
        }
        Ok(false) => {
            return GhOption {
                arg: None,
                content: None,
            }
        }
        Err(_) => {
            panic!("occur some errors")
        }
    }
}
