use super::GhOption;
use inquire::Text;

pub fn build() -> GhOption {
    let t = Text::new("Which repo do you wanna search?")
        .with_help_message("if blank, will search for current directory")
        .prompt()
        .unwrap();

    if t.is_empty() {
        GhOption {
            arg: None,
            content: None,
        }
    } else {
        GhOption {
            arg: Some(String::from("-R")),
            content: Some(t),
        }
    }
}
