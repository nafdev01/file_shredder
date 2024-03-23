use crate::initialize_app::{CustomError, Search};
use notify_rust::Notification as DesktopNotification;
use tokio_postgres::NoTls;
use regex::Regex;
use walkdir::WalkDir;

#[tauri::command]
pub async fn log_search(
    searcher: &i32,
    pattern: &String,
    directory: &String,
    files_found: &i32,
) -> Result<(), CustomError> {
    let (client, connection) = tokio_postgres::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    ).await?;

    tokio::spawn(async move {
        if let Err(_) = connection.await {
            DesktopNotification::new()
            .summary("SFS")
            .body(&"Error connecting to server. Check your internet connection and try again".to_string())
            .show()
            .unwrap();
        }
    });

    client.execute(
        "INSERT INTO searches (searcher,word, directory, no_of_files) VALUES ($1, $2, $3, $4)",
        &[searcher, pattern, directory, files_found],
    ).await?;

    Ok(())
}

#[tauri::command]
pub async fn find_files(pattern: String, directory: String, searcher: i32) -> Result<Vec<String>, CustomError> {
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

    match log_search(&searcher, &pattern, &directory, &num_results).await {
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
        Err(_) => {
            DesktopNotification::new()
                .summary("SFS")
                .body(&"Error connecting to server. Check your internet connection and try again".to_string())
                .show()
                .unwrap();
        }
    };

    Ok(files)
}

#[tauri::command]
pub async fn get_search_history(searcher: i32) -> Result<Vec<Search>, CustomError> {
    let (client, connection) = tokio_postgres::connect(
        "postgresql://priestley:PassMan2024@64.23.233.35/shredder",
        NoTls,
    ).await?;

    tokio::spawn(async move {
        if let Err(_) = connection.await {
            DesktopNotification::new()
                .summary("SFS")
                .body(&"Error connecting to server. Check your internet connection and try again".to_string())
                .show()
                .unwrap();
        }
    });

    let rows = client.query(
        "SELECT word, directory, no_of_files, TO_CHAR(searched_at, 'YYYY/MM/DD HH12:MM:SS') AS search_date from searches 
        WHERE searcher = $1",
        &[&searcher],
    ).await?;

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
