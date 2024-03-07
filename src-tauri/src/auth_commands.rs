use crate::initialize_app::CustomError;
use crate::initialize_app::Employee;
use crate::initialize_app::Department;
use crate::initialize_app::Admin;
use sha1::Digest;


#[tauri::command]
pub fn get_departments() -> Result<Vec<Department>, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT department_id, department_name, created_at from departments"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Department {
            department_id: row.get(0)?,
            department_name: row.get(1)?,
            created_at: row.get(2)?,
        })
    })?;

    let mut departments = Vec::new();
    for department in rows {
        departments.push(department?);
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
    password: String
) -> Result<(), CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;
    
    conn.execute(
        "INSERT INTO employees (fullname, username, email, phone, department, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &[&fullname, &username, &email, &phone, &department, &hex::encode(sha1::Sha1::digest(password.as_bytes()))]
    )?;

    Ok(())
}

#[tauri::command]
pub fn authenticate_employee(username: String, password: String) -> Result<Employee, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT employeeid, fullname, username, email, phone, department, created_at 
        FROM employees 
        WHERE username = ?1 AND password = ?2"
    )?;

    let mut user_iter = stmt.query_map(&[&username, &hex::encode(sha1::Sha1::digest(password.as_bytes()))], |row| {
        Ok(Employee {
            employeeid: row.get(0)?,
            fullname: row.get(1)?,
            username: row.get(2)?,
            email: row.get(3)?,
            phone: row.get(4)?,
            department: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;

    if let Some(user) = user_iter.next() {
        Ok(user?)
    } else {
        Err(CustomError::DatabaseError(rusqlite::Error::QueryReturnedNoRows))
    }
}

#[tauri::command]
pub fn authenticate_admin(username: String, password: String) -> Result<Admin, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT adminid, fullname, username, email, phone, department, created_at 
        FROM admins 
        WHERE username = ?1 AND password = ?2"
    )?;

    let mut user_iter = stmt.query_map(&[&username, &hex::encode(sha1::Sha1::digest(password.as_bytes()))], |row| {
        Ok(Admin {
            adminid: row.get(0)?,
            fullname: row.get(1)?,
            username: row.get(2)?,
            email: row.get(3)?,
            phone: row.get(4)?,
            department: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;

    if let Some(user) = user_iter.next() {
        Ok(user?)
    } else {
        Err(CustomError::DatabaseError(rusqlite::Error::QueryReturnedNoRows))
    }
}
