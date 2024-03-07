use crate::initialize_app::Admin;
use crate::initialize_app::CustomError;
use crate::initialize_app::Employee;
use sha1::Digest;

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

#[tauri::command]
pub fn change_employee_password(
    employeeid: String,
    oldpassword: String,
    newpassword: String,
) -> Result<(), CustomError> {
    if oldpassword == newpassword {
        return Err(CustomError::AuthenticationError(
            "New password must be different from the old password".to_string(),
        ));
    }

    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT password 
        FROM employees 
        WHERE employeeid = ?1",
    )?;

    let mut user_iter = stmt.query_map(&[&employeeid], |row| Ok(row.get(0)?))?;

    if let Some(user) = user_iter.next() {
        let password: String = user?;
        let oldhashed: String= hex::encode(sha1::Sha1::digest(oldpassword.as_bytes()));
        if password ==  oldhashed{
            conn.execute(
                "UPDATE employees 
                SET password = ?1
                WHERE employeeid = ?2",
                &[&hex::encode(sha1::Sha1::digest(newpassword.as_bytes())), &employeeid],
            )?;
            Ok(())
        } else {
            Err(CustomError::AuthenticationError(
                "Incorrect password entered".to_string(),
            ))
        }
    } else {
        Err(CustomError::DatabaseError(
            rusqlite::Error::QueryReturnedNoRows,
        ))
    }
}

#[tauri::command]
pub fn update_admin(
    adminid: String,
    fullname: String,
    username: String,
    email: String,
    phone: String,
) -> Result<(), CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    conn.execute(
        "UPDATE admins 
        SET fullname = ?1, username = ?2, email = ?3, phone = ?4
        WHERE adminid = ?5",
        &[&fullname, &username, &email, &phone, &adminid],
    )?;

    Ok(())
}

#[tauri::command]
pub fn change_admin_password(
    adminid: String,
    oldpassword: String,
    newpassword: String,
) -> Result<(), CustomError> {
    if oldpassword == newpassword {
        return Err(CustomError::AuthenticationError(
            "New password must be different from the old password".to_string(),
        ));
    }

    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT password 
        FROM admins 
        WHERE adminid = ?1",
    )?;

    let mut user_iter = stmt.query_map(&[&adminid], |row| Ok(row.get(0)?))?;

    if let Some(user) = user_iter.next() {
        let password: String = user?;
        let oldhashed: String= hex::encode(sha1::Sha1::digest(oldpassword.as_bytes()));
        if password == oldhashed {
            conn.execute(
                "UPDATE admins 
                SET password = ?1
                WHERE adminid = ?2",
                &[&hex::encode(sha1::Sha1::digest(newpassword.as_bytes())), &adminid],
            )?;
            Ok(())
        } else {
            Err(CustomError::AuthenticationError(
                "Incorrect password entered".to_string(),
            ))
        }
    } else {
        Err(CustomError::DatabaseError(
            rusqlite::Error::QueryReturnedNoRows,
        ))
    }
}
