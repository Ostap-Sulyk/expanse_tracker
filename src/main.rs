#![allow(unused)]

use std::fmt::format;

use chrono::prelude::*;
use chrono::NaiveDate;
use inquire::{ui::RenderConfig, CustomType, DateSelect};
//TODO add expense
// get input for date,
// get input for amount
//

fn get_date() {
    struct Today{
        year: i32,
        month: u32,
        day: u32,
    }
    impl Today {
        fn new() -> Today {
            let now = chrono::Utc::now();
            Today{
                year: now.year(),
                month: now.month(),
                day: now.day(),
            }
        }
    }
    let today = Today::new();
    let date = DateSelect::new("When expense happened?")
    .with_default(NaiveDate::from_ymd(today.year, today.month, today.day))
        .with_min_date(NaiveDate::from_ymd(today.year -1, today.month, today.day))
        .with_max_date(NaiveDate::from_ymd(today.year +1, today.month, today.day))
        .prompt();
    
}
fn get_amount() {
    let amount_prompt = CustomType::<f64>::new("Enter amount: ")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("type amount in us dollars using . as separator")
        .prompt();

    let amount:f64 = amount_prompt.unwrap_or_default();
    println!("{}", amount);


}

fn main() {
    get_date()
}
//TODO get all expenses
