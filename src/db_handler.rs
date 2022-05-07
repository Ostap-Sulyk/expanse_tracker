#![allow(unused)]
use rusqlite::params;
use rusqlite::{Connection, Result};

pub fn setup() -> Result<()> {
    let conn = Connection::open("expense_tracker.db")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS category (
            name TEXT NOT NULL UNIQUE,
            PRIMARY KEY (name)
        )",
        [],
    );
    conn.execute(
        "CREATE TABLE IF NOT EXISTS expense (
            id INTEGER NOT NULL UNIQUE,
            amount INTEGER NOT NULL,
            date TEXT NOT NULL,
            category_name TEXT NON NULL REFERENCES category(name),
            PRIMARY KEY(id AUTOINCREMENT),
            FOREIGN KEY (category_name) references category(name)
        )",
        [],
    );

    Ok(())
}

pub fn insert_category(name: &str) -> Result<()> {
    let conn = Connection::open("expense_tracker.db")?;
    let mut stmt = conn.prepare("INSERT INTO category (name) VALUES (?)")?;
    stmt.execute(params![name])?;
    Ok(())
}
pub fn insert_expense(amount: f64, date: &str, category_name: &str) -> Result<()> {
    let conn = Connection::open("expense_tracker.db")?;
    let mut stmt =
        conn.prepare("INSERT INTO expense (amount, date, category_name) VALUES (?1,?2,?3)")?;
    stmt.execute(params![(amount * 100.0) as i64, date, category_name])?;
    Ok(())
}