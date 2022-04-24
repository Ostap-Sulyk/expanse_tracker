#![allow(unused)]
extern crate chrono;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Expance {
    amount: f64,
    date: Date<Local>,
}

impl Expance {
    pub fn new(amount: f64, y: i32, m: u32, d: u32) -> Expance {
        Expance {
            amount,
            date: Local.ymd(y, m, d),
        }
    }


    pub fn get_date(&self)-> Date<Local>{
        return self.date;
    }
}
