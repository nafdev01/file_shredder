use serde::Serialize;
use tauri::InvokeError;
use rusqlite::Error as RusqliteError;

#[derive(Debug)]
pub enum CustomError {
    DatabaseError(RusqliteError),
    // Add other kinds of errors as needed
}

impl From<RusqliteError> for CustomError {
    fn from(error: RusqliteError) -> Self {
        CustomError::DatabaseError(error)
    }
}

impl Into<InvokeError> for CustomError {
    fn into(self) -> InvokeError {
        match self {
            CustomError::DatabaseError(err) => InvokeError::from(err.to_string()),
            // Handle other kinds of errors as needed
        }
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    user_id: i32,
    full_name: String,
    username: String,
    email: String,
    phone_no: String,
    role: String,
    created_at: String,
}

#[tauri::command]
pub fn create_user(
    full_name: String,
    username: String,
    email: String,
    phone_no: String,
    password: String,
    role: String
) -> Result<(), CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS employees (
            user_id INTEGER PRIMARY KEY,
            full_name TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            phone_no TEXT,
            password TEXT NOT NULL,
            role TEXT CHECK(role IN ('admin', 'standard')) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        []
    )?;

    conn.execute(
        "INSERT INTO employees (full_name, username, email, phone_no, password, role) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &[&full_name, &username, &email, &phone_no, &password, &role]
    )?;

    Ok(())
}

#[tauri::command]
pub fn authenticate_user(username: String, password: String) -> Result<User, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT user_id, full_name, username, email, phone_no, role, created_at 
        FROM employees 
        WHERE username = ?1 AND password = ?2"
    )?;

    let mut user_iter = stmt.query_map(&[&username, &password], |row| {
        Ok(User {
            user_id: row.get(0)?,
            full_name: row.get(1)?,
            username: row.get(2)?,
            email: row.get(3)?,
            phone_no: row.get(4)?,
            role: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;

    if let Some(user) = user_iter.next() {
        Ok(user?)
    } else {
        Err(CustomError::DatabaseError(rusqlite::Error::QueryReturnedNoRows))
    }
}
