use super::GhOption;
use inquire::Text;

pub fn build() -> GhOption {
    let t = Text::new("Which repo do you wanna search?")
        .with_help_message("To search all repo, just enter with nothing")
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
