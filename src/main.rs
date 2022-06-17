mod expense_manager;
mod utils;

// TODO: select for time frame
fn main() {
    expense_manager::setup().unwrap();
    while utils::main_menu().is_ok() {}
}
