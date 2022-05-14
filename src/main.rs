use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Expense {
    id: u32,
    amount: i32,
    date: String,
}

impl Expense {
    fn new(id: u32, amount: i32, date: String) -> Expense {
        Expense { id, amount, date }
    }
}

fn main() -> Result<()> {
    let conn = Connection::open("expense.db")?;
    conn.execute(
        "create table if not exists expense (
            id integer primary key autoincrement,
            amount integer not null,
            date text
        )",
        [],
    )?;

    //    conn.execute(
    //        "insert into expense (amount, date) values(?1, ?2)",
    //        params![photography.amount, photography.date],
    //    )?;

    let mut stmt = conn.prepare("select * from expense")?;
    let expense_iter = stmt.query_map([], |row| {
        Ok(Expense::new(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    println!("id\tamount\tdate");
    for exp in expense_iter {
        println!(
            "{}\t{}\t{}",
            exp.as_ref().unwrap().id,
            exp.as_ref().unwrap().amount,
            exp.as_ref().unwrap().date
        );
    }

    Ok(())
}
