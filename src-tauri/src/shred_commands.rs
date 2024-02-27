use walkdir::WalkDir;
use regex::Regex;
use tauri::command;
use dirs;
use tokio::task;

#[command]
pub async fn find_files(pattern: String) -> Vec<String> {
    let re = Regex::new(&pattern).unwrap();
    let mut files = Vec::new();

    let dir = dirs::home_dir().unwrap();

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if re.is_match(file_name) {
                files.push(String::from(file_name));
            }
        }
    }

    files
}
