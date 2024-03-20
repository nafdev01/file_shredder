use crate::initialize_app::{CustomError, Search};
use notify_rust::Notification as DesktopNotification;
use postgres::{Client, NoTls};
use regex::Regex;
use walkdir::WalkDir;

fn log_search(
    searcher: &i32,
    pattern: &String,
    directory: &String,
    files_found: &i32,
) -> Result<(), CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    client.execute(
        "INSERT INTO searches (searcher,word, directory, no_of_files) VALUES ($1, $2, $3, $4)",
        &[searcher, pattern, directory, files_found],
    )?;

    Ok(())
}

#[tauri::command]
pub fn find_files(pattern: String, directory: String, searcher: i32) -> Vec<String> {
    let re = Regex::new(&pattern).unwrap();
    let mut files = Vec::new();

    for entry in WalkDir::new(&directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_path = path.to_string_lossy();
            if re.is_match(&file_path) {
                files.push(String::from(file_path));
            }
        }
    }

    let num_results = files.len() as i32;
    let search_term = pattern.clone();
    let directory_searched = &directory;

    match log_search(&searcher, &pattern, &directory, &num_results) {
        Ok(_) => {
            DesktopNotification::new()
                .summary("SFS")
                .body(&format!(
                    "Your search for {} in {} returned {} results",
                    search_term, directory_searched, num_results
                ))
                .show()
                .unwrap();
        }
        Err(e) => {
            DesktopNotification::new()
                .summary("SFS")
                .body(&e.to_string())
                .show()
                .unwrap();
        }
    };

    files
}

#[tauri::command]
pub fn get_search_history(searcher: i32) -> Result<Vec<Search>, CustomError> {
    let mut client = Client::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    )?;

    let rows = client.query(
        "SELECT word, directory, no_of_files, TO_CHAR(searched_at, 'YYYY/MM/DD HH12:MM:SS') AS search_date from searches 
        WHERE searcher = $1",
        &[&searcher],
    )?;

    let mut search_history = Vec::new();

    for row in &rows {
        let search = Search {
            searchid: searcher,
            searcher: (&"searcher").to_string(),
            word: row.get(0),
            directory: row.get(1),
            no_of_files: row.get(2),
            searched_at: row.get(3),
        };
        search_history.push(search);
    }

    Ok(search_history)
}
