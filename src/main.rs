#![allow(unused)]
use chrono::{NaiveDate, Weekday};
use inquire::DateSelect;

fn add_expense() {
    let date = DateSelect::new("When expense happened")
        .with_default(NaiveDate::from_ymd(2021, 8, 1))
        .with_min_date(NaiveDate::from_ymd(2021, 8, 1))
        .with_max_date(NaiveDate::from_ymd(2021, 12, 31))
        .with_week_start(Weekday::Mon)
        .with_help_message("Possible flight will be displayed accordingly")
        .prompt();
}
fn main() {
    //TODO add expense
    add_expense()
    //TODO get all expenses
}
