use rusqlite::Error as RusqliteError;
use serde::Deserialize;
use serde::Serialize;
use tauri::InvokeError;

#[derive(Debug)]
pub enum CustomError {
    DatabaseError(RusqliteError),
    AuthenticationError(String),
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
            CustomError::AuthenticationError(err) => InvokeError::from(err),
            // Handle other kinds of errors as needed
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub employeeid: i32,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub department: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub adminid: i32,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub department: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct Department {
    pub department_id: i32,
    pub department_name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub searchid: i32,
    pub searcher: String,
    pub word: String,
    pub directory: String,
    pub no_of_files: i32,
    pub searched_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShredRequest {
    pub requestid: i32,
    pub requestby: String,
    pub filepath: String,
    pub department: String,
    pub requestto: String,
    pub requeststatus: String,
    pub requestat: String,
}

// write code that initializes the database and creates the tables needed for the application.
pub fn initialize_database() -> Result<(), CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS departments (
            department_id INTEGER PRIMARY KEY AUTOINCREMENT,
            department_name TEXT NOT NULL UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS employees (
            employeeid INTEGER PRIMARY KEY AUTOINCREMENT,
            fullname TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            phone TEXT UNIQUE,
            password TEXT NOT NULL,
            department TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (department) REFERENCES departments(department_name)
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS admins (
            adminid INTEGER PRIMARY KEY AUTOINCREMENT,
            fullname TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            phone TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            department TEXT NOT NULL UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (department) REFERENCES departments(department_name)
        );",
        [],
    )?;

    // add some initial data to the departments table
    conn.execute(
        "INSERT OR IGNORE INTO departments (department_name) VALUES (?1), (?2), (?3), (?4), (?5)",
        &[
            "Human Resources",
            "Finance",
            "Marketing",
            "Sales",
            "Operations",
        ],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS searches (
            searchid INTEGER PRIMARY KEY AUTOINCREMENT,
            searcher TEXT NOT NULL,
            word TEXT NOT NULL,
            directory TEXT NOT NULL,
            no_of_files INTEGER NOT NULL,
            searched_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (searcher) REFERENCES employees(employeeid)
        );",
        [],
    )?;

    // create table shredrequests
    conn.execute(
        "CREATE TABLE IF NOT EXISTS shredrequests (
            requestid INTEGER PRIMARY KEY AUTOINCREMENT,
            requestby TEXT NOT NULL,
            filepath TEXT NOT NULL,
            department TEXT NOT NULL,
            requestto TEXT NOT NULL,
            requeststatus TEXT CHECK(requeststatus IN ('Pending', 'Approved', 'Denied')) DEFAULT 'Pending',
            deleted BOOLEAN DEFAULT 0,
            requestat DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (requestby) REFERENCES employees(employeeid),
            FOREIGN KEY (department) REFERENCES departments(department_name),
            FOREIGN KEY (requestto) REFERENCES admins(adminid)
        );",
        [],
    )?;

    // add some default admins for each department
    let departments = [
        "Human Resources",
        "Finance",
        "Marketing",
        "Sales",
        "Operations",
    ];
    for (i, department) in departments.iter().enumerate() {
        conn.execute(
            "INSERT OR IGNORE INTO admins (fullname, username, email, phone, password, department) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            &[
                &format!("Default Admin {}", department),
                &format!("admin_{}", i),
                &format!("admin_{}@company.com", i),
                &format!("123456789{}", i),
                &format!("{}", "Password*2001"),
                &format!("{}", department),
            ]
        )?;
    }

    Ok(())
}
