use crate::initialize_app::{CustomError, Search};
use crate::shredder_functions::log_search;
use notify_rust::Notification as DesktopNotification;
use regex::Regex;
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
