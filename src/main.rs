#![allow(unused)]

use std::fmt::format;

use chrono::prelude::*;
use inquire::{ui::RenderConfig, CustomType};
//TODO add expense
// get input for date,
// get input for amount
//
fn add_expense() {
    let amount_prompt = CustomType::<f64>::new("Enter amount: ")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("type amount in us dollars using . as separator")
        .prompt();
}

fn main() {
    add_expense()
}
//TODO get all expenses
