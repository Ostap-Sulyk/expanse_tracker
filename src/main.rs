#![allow(unused)]

mod expense_manager;
mod utils;
use expense_manager::Expense;


// TODO: select for time frame
// TODO: generate report for time frame
fn main() {
    loop {
        match utils::main_menu() {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
