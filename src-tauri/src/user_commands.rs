use crate::initialize_app::Admin;
use crate::initialize_app::CustomError;
use crate::initialize_app::Employee;
use postgres::{Client, NoTls};
use sha1::Digest;

#[tauri::command]
pub fn get_employee(username: String) -> Result<Employee, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let rows = client.query(
        "SELECT employeeid, fullname, username, email, phone, department 
        FROM employees 
        WHERE username = $1",
        &[&username],
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
pub fn get_admin(username: String) -> Result<Admin, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let rows = client.query(
        "SELECT adminid, fullname, username, email, phone, department, created_at 
        FROM admins 
        WHERE username = $1",
        &[&username],
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

#[tauri::command]
pub fn update_employee(
    employeeid: i32,
    fullname: String,
    username: String,
    email: String,
    phone: String,
) -> Result<(), CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    client.execute(
        "UPDATE employees 
        SET fullname = $1, username = $2, email = $3, phone = $4
        WHERE employeeid = $5",
        &[&fullname, &username, &email, &phone, &employeeid],
    )?;

    Ok(())
}

#[tauri::command]
pub fn change_employee_password(
    employeeid: i32,
    oldpassword: String,
    newpassword: String,
) -> Result<(), CustomError> {
    if oldpassword == newpassword {
        return Err(CustomError::AuthenticationError(
            "New password must be different from the old password".to_string(),
        ));
    }

    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let user_iter = client.query(
        "SELECT password 
        FROM employees 
        WHERE employeeid = $1",
        &[&employeeid],
    )?;

    if let Some(user) = user_iter.iter().next() {
        let password: Option<String> = user.get::<_, Option<String>>("password");
        match password {
            Some(password) => {
                let oldhashed: String = hex::encode(sha1::Sha1::digest(oldpassword.as_bytes()));
                if password == oldhashed {
                    client.execute(
                        "UPDATE employees 
                        SET password = $1
                        WHERE employeeid = $2",
                        &[
                            &hex::encode(sha1::Sha1::digest(newpassword.as_bytes())),
                            &employeeid,
                        ],
                    )?;
                    Ok(())
                } else {
                    Err(CustomError::AuthenticationError(
                        "Incorrect password entered".to_string(),
                    ))
                }
            }
            None => Err(CustomError::AuthenticationError(
                "Incorrect password entered".to_string(),
            )),
        }
    } else {
        Err(CustomError::AuthenticationError(
            "Incorrect password entered".to_string(),
        ))
    }
}

#[tauri::command]
pub fn update_admin(
    adminid: i32,
    fullname: String,
    username: String,
    email: String,
    phone: String,
) -> Result<(), CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    client.execute(
        "UPDATE admins 
        SET fullname = $1, username = $2, email = $3, phone = $4
        WHERE adminid = $5",
        &[&fullname, &username, &email, &phone, &adminid],
    )?;

    Ok(())
}

#[tauri::command]
pub fn change_admin_password(
    adminid: i32,
    oldpassword: String,
    newpassword: String,
) -> Result<(), CustomError> {
    if oldpassword == newpassword {
        return Err(CustomError::AuthenticationError(
            "New password must be different from the old password".to_string(),
        ));
    }

    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let user_iter = client.query(
        "SELECT password 
        FROM admins 
        WHERE adminid = $1",
        &[&adminid],
    )?;

    if let Some(user) = user_iter.iter().next() {
        let password: Option<String> = user.get::<_, Option<String>>("password");
        match password {
            Some(password) => {
                let oldhashed: String = hex::encode(sha1::Sha1::digest(oldpassword.as_bytes()));
                if password == oldhashed {
                    client.execute(
                        "UPDATE admins 
                        SET password = $1
                        WHERE adminid = $2",
                        &[
                            &hex::encode(sha1::Sha1::digest(newpassword.as_bytes())),
                            &adminid,
                        ],
                    )?;
                    Ok(())
                } else {
                    Err(CustomError::AuthenticationError(
                        "Incorrect password entered".to_string(),
                    ))
                }
            }
            None => Err(CustomError::AuthenticationError(
                "Incorrect password entered".to_string(),
            )),
        }
    } else {
        Err(CustomError::AuthenticationError(
            "Incorrect password entered".to_string(),
        ))
    }
}
