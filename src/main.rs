#![allow(unused)]

mod db_handler;
mod expanse;
mod menu;

use expanse::Expanse;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    //db_handler::setup()?;
    
    db_handler::get_all_categories();

    Ok(())
}
