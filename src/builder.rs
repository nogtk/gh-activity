mod option_builder;

pub fn gh_build() -> String {
    let mut command = "gh pr list".to_string();

    command = format!("{} {}", command, option_builder::date().format());
    command = format!("{} {}", command, option_builder::repo().format());
    command = format!("{} {}", command, option_builder::author().format());
    command = format!("{} {}", command, option_builder::limit().format());

    command
}
