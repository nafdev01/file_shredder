use crate::initialize_app::ShredRequest;
use crate::initialize_app::{CustomError};
use rusqlite::{params, Connection};
use std::sync::Arc;


#[tauri::command]
pub fn create_shred_request(requestby: String, filepath: String) -> Result<String, String> {
    let conn = match Connection::open("shredder.db") {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };

    let res = conn.execute(
        "INSERT INTO shredrequests (requestby, filepath, department, requestto) VALUES (?1, ?2, (SELECT department FROM employees WHERE employeeid = ?1), (SELECT adminid FROM admins WHERE department = (SELECT department FROM employees WHERE employeeid = ?1)))",
        params![requestby, filepath],
    );

    match res {
        Ok(_) => Ok("Shred request created successfully".to_string()),
        Err(e) => Err(format!("Failed to create shred request: {}", e)),
    }
}

#[tauri::command]
pub fn get_pending_shred_requests(requestto: String) -> Result<Vec<ShredRequest>, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT requestid, requestby, filepath, department, requeststatus, requestat from shredrequests 
        WHERE requestto = ?1 and requeststatus = 'Pending'",
    )?;

    let requestadmin = Arc::new(requestto.clone());

    let shred_request_iter = stmt.query_map(&[&requestto], |row| {
        let requestadmin = Arc::clone(&requestadmin);
        Ok(ShredRequest {
            requestid: row.get(0)?,
            requestby: row.get(1)?,
            filepath: row.get(2)?,
            department: row.get(3)?,
            requestto: (*requestadmin).clone(),
            requeststatus: row.get(4)?,
            requestat: row.get(5)?,
        })
    })?;

    let mut shredrequests = Vec::new();
    for shredrequest in shred_request_iter {
        shredrequests.push(shredrequest?);
    }

    Ok(shredrequests)
}

#[tauri::command]
pub fn get_denied_shred_requests(requestto: String) -> Result<Vec<ShredRequest>, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT requestid, requestby, filepath, department, requeststatus, requestat from shredrequests 
        WHERE requestto = ?1 and requeststatus = 'Denied'",
    )?;

    let requestadmin = Arc::new(requestto.clone());

    let shred_request_iter = stmt.query_map(&[&requestto], |row| {
        let requestadmin = Arc::clone(&requestadmin);
        Ok(ShredRequest {
            requestid: row.get(0)?,
            requestby: row.get(1)?,
            filepath: row.get(2)?,
            department: row.get(3)?,
            requestto: (*requestadmin).clone(),
            requeststatus: row.get(4)?,
            requestat: row.get(5)?,
        })
    })?;

    let mut shredrequests = Vec::new();
    for shredrequest in shred_request_iter {
        shredrequests.push(shredrequest?);
    }

    Ok(shredrequests)
}

#[tauri::command]
pub fn get_approved_shred_requests(requestto: String) -> Result<Vec<ShredRequest>, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT requestid, requestby, filepath, department, requeststatus, requestat from shredrequests 
        WHERE requestto = ?1 and requeststatus = 'Approved'",
    )?;

    let requestadmin = Arc::new(requestto.clone());

    let shred_request_iter = stmt.query_map(&[&requestto], |row| {
        let requestadmin = Arc::clone(&requestadmin);
        Ok(ShredRequest {
            requestid: row.get(0)?,
            requestby: row.get(1)?,
            filepath: row.get(2)?,
            department: row.get(3)?,
            requestto: (*requestadmin).clone(),
            requeststatus: row.get(4)?,
            requestat: row.get(5)?,
        })
    })?;

    let mut shredrequests = Vec::new();
    for shredrequest in shred_request_iter {
        shredrequests.push(shredrequest?);
    }

    Ok(shredrequests)
}

#[tauri::command]
pub fn update_shred_request(requestid: String, requeststatus: String) -> Result<String, String> {
    let conn = match Connection::open("shredder.db") {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };

    let res = conn.execute(
        "UPDATE shredrequests SET requeststatus = ?1 WHERE requestid = ?2",
        params![requeststatus, requestid],
    );

    match res {
        Ok(_) => Ok("Shred request updated successfully".to_string()),
        Err(e) => Err(format!("Failed to update shred request: {}", e)),
    }
}


#[tauri::command]
pub fn get_employee_denied_shred_requests(requestby: String) -> Result<Vec<ShredRequest>, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT requestid, requestto, filepath, department, requeststatus, requestat from shredrequests 
        WHERE requestby = ?1 and requeststatus = 'Denied'",
    )?;

    let requestemployee = Arc::new(requestby.clone());

    let shred_request_iter = stmt.query_map(&[&requestby], |row| {
        let requestemployee = Arc::clone(&requestemployee);
        Ok(ShredRequest {
            requestid: row.get(0)?,
            requestby: (*requestemployee).clone(),
            filepath: row.get(2)?,
            department: row.get(3)?,
            requestto: row.get::<_, i32>(1)?.to_string(), // change here
            requeststatus: row.get(4)?,
            requestat: row.get(5)?,
        })
    })?;

    let mut shredrequests = Vec::new();
    for shredrequest in shred_request_iter {
        shredrequests.push(shredrequest?);
    }

    Ok(shredrequests)
}

#[tauri::command]
pub fn get_employee_approved_shred_requests(requestby: String) -> Result<Vec<ShredRequest>, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT requestid, requestto, filepath, department, requeststatus, requestat from shredrequests 
        WHERE requestby = ?1 and requeststatus = 'Approved'",
    )?;

    let requestemployee = Arc::new(requestby.clone());

    let shred_request_iter = stmt.query_map(&[&requestby], |row| {
        let requestemployee = Arc::clone(&requestemployee);
        Ok(ShredRequest {
            requestid: row.get(0)?,
            requestby: (*requestemployee).clone(),
            filepath: row.get(2)?,
            department: row.get(3)?,
            requestto: row.get::<_, i32>(1)?.to_string(), // change here
            requeststatus: row.get(4)?,
            requestat: row.get(5)?,
        })
    })?;

    let mut shredrequests = Vec::new();
    for shredrequest in shred_request_iter {
        shredrequests.push(shredrequest?);
    }

    Ok(shredrequests)
}