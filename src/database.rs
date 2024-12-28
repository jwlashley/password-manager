use rusqlite::{Connection, Result as SqlResult};
use crate::account::Account;

// Constants for database configuration
const DB_PATH: &str = "passwords.db";

pub fn init_db() -> SqlResult<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            service_name TEXT NOT NULL,
            username TEXT NOT NULL,
            encrypted_password BLOB NOT NULL,
            nonce BLOB NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn save_account(account: &Account) -> SqlResult<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "INSERT INTO accounts (service_name, username, encrypted_password, nonce)
         VALUES (?1, ?2, ?3, ?4)",
        (
            &account.service_name,
            &account.username,
            &account.encrypted_password,
            &account.nonce,
        ),
    )?;

    Ok(())
}

pub fn find_accounts_by_service(service_name: &str) -> SqlResult<Vec<Account>> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare(
        "SELECT service_name, username, encrypted_password, nonce
         FROM accounts
         WHERE service_name = ?1"
    )?;

    let accounts = stmt.query_map([service_name], |row| {
        Ok(Account {
            service_name: row.get(0)?,
            username: row.get(1)?,
            encrypted_password: row.get(2)?,
            nonce: row.get(3)?,
        })
    })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(accounts)
}

// Could add more database operations like:
// pub fn delete_account(service_name: &str, username: &str) -> SqlResult<()>
// pub fn update_account_password(account: &Account) -> SqlResult<()>
// pub fn list_all_services() -> SqlResult<Vec<String>>