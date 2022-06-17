use ansi_term::Colour;
use std::cmp::Ordering;

use rusqlite::{params, Connection, Result};

pub struct Expense {
    id: u32,
    amount: i64,
    date: String,
}

impl Expense {
    pub fn new(id: Option<u32>, amount: i64, date: String) -> Expense {
        match id {
            Some(id) => Expense { id, amount, date },
            None => Expense {
                id: 0,
                amount,
                date,
            },
        }
    }
    pub fn get_amount_str(&self) -> String {
        format!("{:.2}", self.amount as f64 / 100.0)
    }
}

pub fn setup() -> Result<()> {
    let conn = Connection::open("expense.db")?;
    conn.execute(
        "create table if not exists expense (
            id integer primary key autoincrement,
            amount integer not null,
            date text
        )",
        [],
    )?;

    Ok(())
}
pub fn add_expense(amount: i64, date: String) -> Result<()> {
    let conn = Connection::open("expense.db")?;
    conn.execute(
        "insert into expense (amount, date) values(?1, ?2)",
        params![amount, date],
    )?;

    Ok(())
}

// printing staf to the terminal
pub fn generate_report() -> Result<()> {
    let conn = Connection::open("expense.db")?;
    let mut stmt = conn.prepare("select * from expense")?;
    let expense_iter = stmt.query_map([], |row| {
        Ok(Expense::new(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;
    println!("id\t\tamount\t\tdate");
    for exp in expense_iter {
        let x = Expense::new(
            Some(exp.as_ref().unwrap().id),
            exp.as_ref().unwrap().amount,
            exp.as_ref().unwrap().date.clone(),
        );
        match x.amount.cmp(&0) {
            Ordering::Less => {
                println!(
                    "{}\t\t${:.2}\t\t{}",
                    x.id,
                    Colour::Red.paint(x.get_amount_str()),
                    x.date,
                )
            }
            Ordering::Greater => {
                println!(
                    "{}\t\t${:.2}\t\t{}",
                    x.id,
                    Colour::Green.paint(x.get_amount_str()),
                    x.date,
                )
            }
            Ordering::Equal => {
                println!("{}\t\t${:.2}\t\t{}", x.id, x.get_amount_str(), x.date,)
            }
        }
    }
    Ok(())
}
