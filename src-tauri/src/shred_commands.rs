use crate::initialize_app::ShredRequest;
use crate::initialize_app::{CustomError, Search};
use crate::shredder_functions::log_search;
use notify_rust::Notification as DesktopNotification;
use regex::Regex;
use rusqlite::{params, Connection};
use std::sync::Arc;
use walkdir::WalkDir;

#[tauri::command]
pub async fn find_files(pattern: String, directory: String, searcher: String) -> Vec<String> {
    let re = Regex::new(&pattern).unwrap();
    let mut files = Vec::new();

    for entry in WalkDir::new(directory.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if re.is_match(file_name) {
                files.push(String::from(file_name));
            }
        }
    }

    let num_results = files.len() as i32;
    let search_term = pattern.clone();
    let directory_searched = directory.clone();

    match log_search(searcher, pattern, directory, num_results) {
        Ok(_) => {
            DesktopNotification::new()
                .summary("Shredder")
                .body(&format!(
                    "Your search for {} in {} returned {} results",
                    search_term, directory_searched, num_results
                ))
                .show()
                .unwrap();
        }
        Err(e) => {
            DesktopNotification::new()
                .summary("Shredder")
                .body(&e.to_string())
                .show()
                .unwrap();
        }
    };

    files
}

#[tauri::command]
pub fn get_search_history(searcher: String) -> Result<Vec<Search>, CustomError> {
    let conn = rusqlite::Connection::open("shredder.db")?;

    let mut stmt = conn.prepare(
        "SELECT word, directory, no_of_files, searched_at from searches 
        WHERE searcher = ?1",
    )?;

    let searchid = searcher.parse::<i32>().unwrap();

    let search_iter = stmt.query_map(&[&searcher], |row| {
        Ok(Search {
            searchid: searchid,
            searcher: (&"searcher").to_string(),
            word: row.get(0)?,
            directory: row.get(1)?,
            no_of_files: row.get(2)?,
            searched_at: row.get(3)?,
        })
    })?;

    let mut search_history = Vec::new();
    for search in search_iter {
        search_history.push(search?);
    }

    Ok(search_history)
}

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
