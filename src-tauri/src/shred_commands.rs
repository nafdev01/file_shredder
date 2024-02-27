use walkdir::WalkDir;
use regex::Regex;
use tauri::command;
use std::path::Path;

#[command]
pub async fn find_files(pattern: String, dir_path: String) -> Vec<String> {
    let re = Regex::new(&pattern).unwrap();
    let mut files = Vec::new();

    let dir = Path::new(&dir_path);
    // Print the directory path
    println!("Directory: {}", dir.display());

    for entry in WalkDir::new(dir).follow_links(true) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if re.is_match(file_name) {
                        files.push(String::from(file_name));
                    }
                }
            }
            Err(err) => {
                // Handle permission errors (e.g., "Permission denied")
                println!("Error accessing entry: {}", err);
            }
        }
    }

    files
}
