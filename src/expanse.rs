#![allow(unused)]
extern crate chrono;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Expanse {
    amount: f64,
    date: Date<Local>,
}

impl Expanse {
    pub fn new(amount: f64, y: i32, m: u32, d: u32) -> Expanse {
        Expanse {
            amount,
            date: Local.ymd(y, m, d),
        }
    }

    pub fn get_date(&self) -> Date<Local> {
        self.date
    }
}
