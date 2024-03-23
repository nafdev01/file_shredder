use postgres::{Client, Error, NoTls};
use serde::Deserialize;
use serde::Serialize;
use sha1::Digest;
use std::fmt;

use tauri::InvokeError;

#[derive(Debug)]
pub enum CustomError {
    DatabaseError(Error),
    AuthenticationError(String),
    // Add other kinds of errors as needed
}

impl From<Error> for CustomError {
    fn from(error: Error) -> Self {
        CustomError::DatabaseError(error)
    }
}

impl Into<InvokeError> for CustomError {
    fn into(self) -> InvokeError {
        match self {
            CustomError::DatabaseError(err) => InvokeError::from(err.to_string()),
            CustomError::AuthenticationError(err) => InvokeError::from(err),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            CustomError::AuthenticationError(err) => write!(f, "Authentication Error: {}", err),
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub adminid: i32,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub department: String,
}

#[derive(Debug, Serialize)]
pub struct Department {
    pub department_id: i32,
    pub department_name: String,
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
    pub requestby: i32,
    pub filepath: String,
    pub department: String,
    pub requestto: i32,
    pub requeststatus: String,
    pub requestat: String,
}

// write code that initializes the database and creates the tables needed for the application.
pub fn initialize_database() -> Result<(), CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    client.batch_execute(
        "
    CREATE TABLE IF NOT EXISTS departments (
        department_id SERIAL PRIMARY KEY,
        department_name TEXT NOT NULL UNIQUE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)
        ",
    )?;

    client.batch_execute(
        "
    CREATE TABLE IF NOT EXISTS employees (
        employeeid SERIAL PRIMARY KEY,
        fullname TEXT NOT NULL,
        username TEXT NOT NULL UNIQUE,
        email TEXT NOT NULL UNIQUE,
        phone TEXT UNIQUE,
        password TEXT NOT NULL,
        department TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (department) REFERENCES departments(department_name))
    ",
    )?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS admins (
            adminid SERIAL PRIMARY KEY,
            fullname TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            phone TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            department TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (department) REFERENCES departments(department_name))
            ",
    )?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS searches (
            searchid SERIAL PRIMARY KEY,
            searcher INTEGER NOT NULL,
            word TEXT NOT NULL,
            directory TEXT NOT NULL,
            no_of_files INTEGER NOT NULL,
            searched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (searcher) REFERENCES employees(employeeid))
        ",
    )?;

    // create table shredrequests
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS shredrequests (
            requestid SERIAL PRIMARY KEY,
            requestby INTEGER NOT NULL,
            filepath TEXT NOT NULL,
            department TEXT NOT NULL,
            requestto INTEGER NOT NULL,
            requeststatus TEXT CHECK(requeststatus IN ('Pending', 'Approved', 'Denied')) DEFAULT 'Pending',
            deletion_complete TEXT CHECK(deletion_complete IN ('Yes', 'No')) DEFAULT 'No',
            requestat TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (requestby) REFERENCES employees(employeeid),
            FOREIGN KEY (department) REFERENCES departments(department_name),
            FOREIGN KEY (requestto) REFERENCES admins(adminid))
        ")?;

    client.execute(
            "INSERT INTO departments (department_name) VALUES ($1), ($2), ($3), ($4), ($5) ON CONFLICT DO NOTHING",
            &[
                &"Human Resources",
                &"Finance",
                &"Marketing",
                &"Sales",
                &"Operations",
            ],
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
        client.execute(
            "INSERT INTO admins (fullname, username, email, phone, password, department) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
            &[
                &format!("Default Admin {}", department),
                &format!("admin_{}", i),
                &format!("admin_{}@company.com", i),
                &format!("123456789{}", i),
                &format!("{}", &hex::encode(sha1::Sha1::digest("Password*2001".as_bytes()))),
                &format!("{}", department),
            ]
        )?;
    }

    Ok(())
}
