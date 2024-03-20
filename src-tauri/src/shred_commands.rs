use crate::initialize_app::CustomError;
use crate::initialize_app::ShredRequest;
use crate::shred_file::shred_file;
use postgres::{Client, NoTls};

#[tauri::command]
pub fn create_shred_request(requestby: i32, filepath: String) -> Result<(), String> {
    let mut client = match Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    ) {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to connect to the database: {}", e)),
    };

    match client.execute(
        "INSERT INTO shredrequests (requestby, filepath, department, requestto) VALUES ($1, $2, (SELECT department FROM employees WHERE employeeid = $1), (SELECT adminid FROM admins WHERE department = (SELECT department FROM employees WHERE employeeid = $1)))",
        &[&requestby, &filepath],
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to execute query: {}", e)),
    }
}

#[tauri::command]
pub fn get_pending_shred_requests(requestto: i32) -> Result<Vec<ShredRequest>, CustomError> {
        let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let mut shredrequests = Vec::new();

    let rows = client.query(
        "SELECT requestid, requestby, filepath, department, requeststatus, TO_CHAR(requestat, 'YYYY/MM/DD HH12:MM:SS') AS request_date from shredrequests 
        WHERE requestto = $1 and requeststatus = 'Pending'",
        &[&requestto ],
    )?;

    for row in &rows {
        let shredrequest = ShredRequest {
            requestid: row.get(0),
            requestby: row.get(1),
            filepath: row.get(2),
            department: row.get(3),
            requestto: requestto,
            requeststatus: row.get(4),
            requestat: row.get(5),
        };
        shredrequests.push(shredrequest);
    }


    Ok(shredrequests)
}

#[tauri::command]
pub fn get_denied_shred_requests(requestto: i32) -> Result<Vec<ShredRequest>, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let mut shredrequests = Vec::new();

    let rows = client.query(
        "SELECT requestid, requestby, filepath, department, requeststatus, TO_CHAR(requestat, 'YYYY/MM/DD HH12:MM:SS') AS request_date from shredrequests 
        WHERE requestto = $1 and requeststatus = 'Denied'",
        &[&requestto ],
    )?;

    for row in &rows {
        let shredrequest = ShredRequest {
            requestid: row.get(0),
            requestby: row.get(1),
            filepath: row.get(2),
            department: row.get(3),
            requestto: requestto,
            requeststatus: row.get(4),
            requestat: row.get(5),
        };
        shredrequests.push(shredrequest);
    }


    Ok(shredrequests)
}

#[tauri::command]
pub fn get_approved_shred_requests(requestto: i32) -> Result<Vec<ShredRequest>, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let mut shredrequests = Vec::new();

    let rows = client.query(
        "SELECT requestid, requestby, filepath, department, requeststatus, TO_CHAR(requestat, 'YYYY/MM/DD HH12:MM:SS') AS request_date from shredrequests 
        WHERE requestto = $1 and requeststatus = 'Approved'",
        &[&requestto ],
    )?;

    for row in &rows {
        let shredrequest = ShredRequest {
            requestid: row.get(0),
            requestby: row.get(1),
            filepath: row.get(2),
            department: row.get(3),
            requestto: requestto,
            requeststatus: row.get(4),
            requestat: row.get(5),
        };
        shredrequests.push(shredrequest);
    }

    Ok(shredrequests)
}

#[tauri::command]
pub fn update_shred_request(requestid: i32, requeststatus: String) -> Result<String, String> {
    let mut client = match Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    ) {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to connect to the database: {}", e)),
    };

    match client.execute(
        "UPDATE shredrequests SET requeststatus = $1 WHERE requestid = $2",
        &[&requeststatus, &requestid],
    ) {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => Err(format!("Failed to execute query: {}", e)),
    }
}

#[tauri::command]
pub fn get_employee_denied_shred_requests(
    requestby: i32,
) -> Result<Vec<ShredRequest>, CustomError> {
        let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let mut shredrequests = Vec::new();

    let rows = client.query(
        "SELECT requestid, filepath, department, requeststatus, TO_CHAR(requestat, 'YYYY/MM/DD HH12:MM:SS') AS request_date,requestto from shredrequests 
        WHERE requestby = $1 and requeststatus = 'Denied'",
        &[&requestby ],
    )?;

    for row in &rows {
        let shredrequest = ShredRequest {
            requestid: row.get(0),
            requestby: requestby,
            filepath: row.get(1),
            department: row.get(2),
            requestto: row.get(5),
            requeststatus: row.get(3),
            requestat: row.get(4),
        };
        shredrequests.push(shredrequest);
    }

    Ok(shredrequests)
}


#[tauri::command]
pub fn get_employee_approved_shred_requests(
    requestby: i32,
) -> Result<Vec<ShredRequest>, CustomError> {
        let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let mut shredrequests = Vec::new();

    let rows = client.query(
        "SELECT requestid, filepath, department, requeststatus, TO_CHAR(requestat, 'YYYY/MM/DD HH12:MM:SS') AS request_date,requestto from shredrequests 
        WHERE requestby = $1 and requeststatus = 'Approved'",
        &[&requestby ],
    )?;

    for row in &rows {
        let shredrequest = ShredRequest {
            requestid: row.get(0),
            requestby: requestby,
            filepath: row.get(1),
            department: row.get(2),
            requestto: row.get(5),
            requeststatus: row.get(3),
            requestat: row.get(4),
        };
        shredrequests.push(shredrequest);
    }

    Ok(shredrequests)
}

#[tauri::command]
pub fn complete_shred_request(shredfile: String) -> Result<String, String> {
    let path = shredfile; // shredfile is already a String representing the path

    match shred_file(&path) {
        Ok(_) => Ok("success".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
