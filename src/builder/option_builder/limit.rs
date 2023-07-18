use super::GhOption;
use inquire::Text;

pub fn build() -> GhOption {
    let t = Text::new("Maximum number of items to fetch")
        .with_default("30")
        .prompt()
        .unwrap();

    GhOption {
        arg: Some(String::from("--limit")),
        content: Some(t),
    }
}
