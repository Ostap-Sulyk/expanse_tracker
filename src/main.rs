#![allow(unused)]

mod expanse;
use expanse::Expance;

fn main() {
    let amount = 5.5;
    let y = 2022;
    let m = 1;
    let d = 30;
    let mut x = String::new();
    std::io::stdin().read_line(&mut x);

    let date = Expance::new(amount, y, m, d);
}
