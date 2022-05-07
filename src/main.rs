#![allow(unused)]

mod expanse;
use expanse::Expanse;

// fn main() {
//     let amount = 5.5;
//     let y = 2022;
//     let m = 1;
//     let d = 30;
//     let mut x = String::new();
//     std::io::stdin().read_line(&mut x);

//     let date = Expanse::new(amount, y, m, d);
//     println!("hello there")
// }

mod db_handler;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    db_handler::setup()?;
    db_handler::insert_category("photography")?;
    db_handler::insert_expense(3.45, "today", "photography")?;

    Ok(())
}
