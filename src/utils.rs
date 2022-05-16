use chrono::prelude::*;
use chrono::NaiveDate;
use inquire::{ui::RenderConfig, CustomType, DateSelect};

use crate::expense_manager;
use crate::expense_manager::Expense;
use crate::utils;

pub fn get_date() -> String {
    struct Today {
        year: i32,
        month: u32,
        day: u32,
    }
    impl Today {
        fn new() -> Today {
            let now = chrono::Utc::now();
            Today {
                year: now.year(),
                month: now.month(),
                day: now.day(),
            }
        }
    }
    let today = Today::new();

    let date = DateSelect::new("When expense happened?")
        .with_default(NaiveDate::from_ymd(today.year, today.month, today.day))
        .with_min_date(NaiveDate::from_ymd(today.year - 1, today.month, today.day))
        .with_max_date(NaiveDate::from_ymd(today.year + 1, today.month, today.day))
        .prompt();

    let date = date.unwrap();
    date.to_string()
}

pub fn get_amount() -> i64 {
    let amount_prompt = CustomType::<f64>::new("Enter amount: ")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("type amount in us dollars using . as separator")
        .prompt();

    let amount: i64 = (amount_prompt.unwrap_or_default() * 100.0) as i64;
    amount
}

pub fn main_menu() {
    use inquire::{error::InquireError, Select};

    let options: Vec<&str> = vec!["Add Expense", "See Analytics", "Exit"];

    let ans: Result<&str, InquireError> = Select::new("What do you want to do?", options).prompt();

    match ans {
        Ok(choice) => match choice {
            "Add Expense" => {
                let amount = utils::get_amount();
                let date = utils::get_date();
                expense_manager::add_expense(amount, date);

            },
            "See Analytics" => {
                expense_manager::select_everything();
            }
            _ => {
                println!("Bye");
            }
        },
        Err(_) => println!("There was an error, please try again"),
    }
}
