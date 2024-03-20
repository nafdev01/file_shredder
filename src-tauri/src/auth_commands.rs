use crate::initialize_app::Admin;
use crate::initialize_app::CustomError;
use crate::initialize_app::Department;
use crate::initialize_app::Employee;
use postgres::{Client, NoTls};
use sha1::Digest;

#[tauri::command]
pub fn get_departments() -> Result<Vec<Department>, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let rows = client.query(
        "SELECT department_id, department_name from departments",
        &[],
    )?;

    let mut departments = Vec::new();

    for row in &rows {
        let department = Department {
            department_id: row.get(0),
            department_name: row.get(1),
        };
        departments.push(department);
    }

    Ok(departments)
}

#[tauri::command]
pub fn create_employee(
    fullname: String,
    username: String,
    email: String,
    phone: String,
    department: String,
    password: String,
) -> Result<(), CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    client.execute(
        "INSERT INTO employees (fullname, username, email, phone, department, password) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&fullname, &username, &email, &phone, &department, &hex::encode(sha1::Sha1::digest(password.as_bytes()))]
    )?;

    Ok(())
}

#[tauri::command]
pub fn authenticate_employee(username: String, password: String) -> Result<Employee, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let rows = client.query(
        "SELECT employeeid, fullname, username, email, phone, department 
        FROM employees 
        WHERE username = $1 AND password = $2",
        &[
            &username,
            &hex::encode(sha1::Sha1::digest(password.as_bytes())),
        ],
    )?;

    if let Some(row) = rows.iter().next() {
        Ok(Employee {
            employeeid: row.get(0),
            fullname: row.get(1),
            username: row.get(2),
            email: row.get(3),
            phone: row.get(4),
            department: row.get(5),
        })
    } else {
        Err(CustomError::AuthenticationError(
            "Invalid username or password".to_string(),
        ))
    }
}

#[tauri::command]
pub fn authenticate_admin(username: String, password: String) -> Result<Admin, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let rows = client.query(
        "SELECT adminid, fullname, username, email, phone, department 
        FROM admins 
        WHERE username = $1 AND password = $2",
        &[
            &username,
            &hex::encode(sha1::Sha1::digest(password.as_bytes())),
        ],
    )?;

    if let Some(row) = rows.iter().next() {
        Ok(Admin {
            adminid: row.get(0),
            fullname: row.get(1),
            username: row.get(2),
            email: row.get(3),
            phone: row.get(4),
            department: row.get(5),
        })
    } else {
        Err(CustomError::AuthenticationError(
            "Invalid username or password".to_string(),
        ))
    }
}
