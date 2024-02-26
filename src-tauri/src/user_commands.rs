use crate::initialize_app::CustomError;
use crate::initialize_app::Employee;
use crate::initialize_app::Admin;

#[tauri::command]
pub fn get_employee(username: String) -> Result<Employee, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT employee_id, full_name, username, email, phone_no, department, created_at 
        FROM employees 
        WHERE username = ?1"
    )?;

    let mut user_iter = stmt.query_map(&[&username], |row| {
        Ok(Employee {
            employee_id: row.get(0)?,
            full_name: row.get(1)?,
            username: row.get(2)?,
            email: row.get(3)?,
            phone_no: row.get(4)?,
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
pub fn get_admin(username: String) -> Result<Admin, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT admin_id, full_name, username, email, phone_no, department, created_at 
        FROM admins 
        WHERE username = ?1"
    )?;

    let mut user_iter = stmt.query_map(&[&username], |row| {
        Ok(Admin {
            admin_id: row.get(0)?,
            full_name: row.get(1)?,
            username: row.get(2)?,
            email: row.get(3)?,
            phone_no: row.get(4)?,
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
