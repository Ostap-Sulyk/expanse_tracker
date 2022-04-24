#![allow(unused)]

mod expanse;
use expanse::Expance;

fn main() {
    let amount = 5.5;
    let y = 2022;
    let m: u32 = 1;
    let d: u32 = 30;

    let date = Expance::new(amount, y, m, d);

    println!("{:?}", date.get_date());

}
