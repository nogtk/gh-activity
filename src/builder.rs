use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use inquire::{DateSelect, Text};

pub fn gh_build() -> String {
    let now = Local::now().date_naive();
    let two_years_ago = NaiveDate::from_ymd_opt(now.year() - 2, now.month(), now.day()).unwrap();
    let end_of_month = NaiveDate::from_ymd_opt(
        if now.month() == 12 {
            now.year() + 1
        } else {
            now.year()
        },
        if now.month() == 12 {
            1
        } else {
            now.month() + 1
        },
        1,
    )
    .unwrap()
        - Duration::days(1);

    let start_date = DateSelect::new("When do you wanna search from?")
        .with_starting_date(now)
        .with_min_date(two_years_ago)
        .with_max_date(end_of_month)
        .with_week_start(Weekday::Sun)
        .prompt()
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    let end_date = DateSelect::new("When do you wanna search until?")
        .with_starting_date(now)
        .with_min_date(two_years_ago)
        .with_max_date(end_of_month)
        .with_week_start(Weekday::Sun)
        .prompt()
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    let command = "gh pr list --search merged:".to_string();
    let command = format!("{}{}..{}", command, start_date, end_date);

    let repo = Text::new("Which repo do you wanna search?").prompt().unwrap();
    let command = format!("{} -R {}", command, repo);

    let author = Text::new("Who is the author?")
        .with_default("@me")
        .prompt()
        .unwrap();
    let command = format!("{} --author {}", command, author);

    println!("{}", command);
    command
}
