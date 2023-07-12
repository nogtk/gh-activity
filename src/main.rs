use chrono::{NaiveDate, Weekday, Local, Duration, Datelike};
use inquire::{DateSelect, Text};
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut command = "gh".to_owned();
    command.push_str(&" pr list");

    let now = Local::now().date_naive();
    let two_years_ago = NaiveDate::from_ymd_opt(
        now.year() - 2,
        now.month(),
        now.day()
    ).unwrap();
    let end_of_month = NaiveDate::from_ymd_opt(
        if now.month() == 12 { now.year() + 1 } else { now.year() },
        if now.month() == 12 { 1 } else { now.month() + 1 },
        1,
    ).unwrap() - Duration::days(1);

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

    let repo = Text::new("What is repo for searching?").prompt().unwrap();


    let default = "gh pr list --search merged:".to_string();
    let mut args = format!("{}{}", default, start_date);
    args = format!("{}..{}", args, end_date);
    args = format!("{} -R {}", args, repo);

    println!("{}", args);

    let mut gh_process = Command::new("sh")
        .arg("-c")
        .arg(args)
        .spawn()
        .unwrap();

    let _ = gh_process.wait().await;
}
