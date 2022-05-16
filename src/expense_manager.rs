use rusqlite::{params, Connection, Result};

pub struct Expense {
    id: u32,
    amount: f64,
    date: String,
}

impl Expense {
    pub fn new(id: Option<u32>, amount: f64, date: String) -> Expense {
        match id {
            Some(id) => Expense { id, amount, date },
            None => Expense {
                id: 0,
                amount,
                date,
            },
        }
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn date(&self) -> String {
        self.date.clone()
    }
}

fn setup() -> Result<()> {
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

pub fn select_everything() -> Result<()> {
    let conn = Connection::open("expense.db")?;
    let mut stmt = conn.prepare("select * from expense")?;
    let expense_iter = stmt.query_map([], |row| {
        Ok(Expense::new(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;
    println!("id\tamount\tdate");
    for exp in expense_iter {
        println!(
            "{}\t${:.2}\t{}",
            exp.as_ref().unwrap().id,
            exp.as_ref().unwrap().amount / 100.0,
            exp.as_ref().unwrap().date
        );
    }
    Ok(())
}
