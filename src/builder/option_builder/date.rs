use super::GhOption;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use inquire::{DateSelect, Select};

pub fn build() -> GhOption {
    let options: Vec<&str> = vec!["created", "closed", "merged"];
    let selected = Select::new("What type of event do you wanna search?", options)
        .prompt()
        .unwrap()
        .to_string();

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

    GhOption {
        arg: Some(String::from("--search")),
        content: Some(format!("{}:{}..{}", selected, start_date, end_date)),
    }
}
