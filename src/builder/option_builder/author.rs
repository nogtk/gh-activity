use super::GhOption;
use inquire::Text;

pub fn build() -> GhOption {
    let t = Text::new("Who is the author?")
        .with_default("@me")
        .prompt()
        .unwrap();
    GhOption {
        arg: Some(String::from("--author")),
        content: Some(t),
    }
}
