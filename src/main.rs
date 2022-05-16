#![allow(unused)]

mod expense_manager;
mod utils;
use expense_manager::Expense;

// insert
fn main() {
    loop {
        match utils::main_menu() {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
//TODO get all expenses
