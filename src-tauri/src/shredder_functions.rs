use rusqlite::{params, Connection, Result};

// shredder_fucntions.rs
pub fn log_search(
    searcher: String,
    pattern: String,
    directory: &String,
    files_found: i32,
) -> Result<()> {
    let conn = Connection::open("shredder.db")?;

    conn.execute(
        "INSERT INTO searches (searcher,word, directory, no_of_files) VALUES (?1, ?2, ?3, ?4)",
        params![searcher, pattern, directory, files_found],
    )?;

    Ok(())
}


