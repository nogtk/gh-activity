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

    let start_date = DateSelect::new("When do you search from date?")
        .with_starting_date(now)
        .with_min_date(two_years_ago)
        .with_max_date(end_of_month)
        .with_week_start(Weekday::Sun)
        .with_help_message("Possible flights will be displayed according to the selected date")
        .prompt()
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    let end_date = DateSelect::new("When do you search until date?")
        .with_starting_date(now)
        .with_min_date(two_years_ago)
        .with_max_date(end_of_month)
        .with_week_start(Weekday::Sun)
        .with_help_message("Possible flights will be displayed according to the selected date")
        .prompt()
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    let command = "gh pr list --search merged:".to_string();
    let command = format!("{}{}..{}", command, start_date, end_date);

    let repo = Text::new("What is repo for searching?").prompt().unwrap();
    let command = format!("{} -R {}", command, repo);

    command
}
