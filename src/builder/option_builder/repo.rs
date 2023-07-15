use super::GhOption;
use inquire::Text;

pub fn build() -> GhOption {
    let t = Text::new("Which repo do you wanna search?")
        .prompt()
        .unwrap();
    GhOption {
        arg: Some(String::from("-R")),
        content: t,
    }
}
