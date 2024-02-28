use crate::initialize_app::Admin;
use crate::initialize_app::CustomError;
use crate::initialize_app::Employee;

#[tauri::command]
pub fn get_employee(username: String) -> Result<Employee, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT employeeid, fullname, username, email, phone, department, created_at 
        FROM employees 
        WHERE username = ?1",
    )?;

    let mut user_iter = stmt.query_map(&[&username], |row| {
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
        Err(CustomError::DatabaseError(
            rusqlite::Error::QueryReturnedNoRows,
        ))
    }
}

#[tauri::command]
pub fn get_admin(username: String) -> Result<Admin, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT adminid, fullname, username, email, phone, department, created_at 
        FROM admins 
        WHERE username = ?1",
    )?;

    let mut user_iter = stmt.query_map(&[&username], |row| {
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
        Err(CustomError::DatabaseError(
            rusqlite::Error::QueryReturnedNoRows,
        ))
    }
}

#[tauri::command]
pub fn update_employee(
    employeeid: String,
    fullname: String,
    username: String,
    email: String,
    phone: String,
) -> Result<(), CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    conn.execute(
        "UPDATE employees 
        SET fullname = ?1, username = ?2, email = ?3, phone = ?4
        WHERE employeeid = ?5",
        &[&fullname, &username, &email, &phone, &employeeid],
    )?;

    Ok(())
}
