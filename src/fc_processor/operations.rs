use colored::{Color::BrightBlack, Colorize};
use rusqlite::{Connection, Result};
use std::path::PathBuf;
pub fn show_duplicates(path: &str) -> Result<()> {
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare(
        "SELECT hash, GROUP_CONCAT(path, '|')
         FROM files
         GROUP BY hash
         HAVING COUNT(*) > 1",
    )?;
    for row in statement.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))? {
        let mut line_counter = 0u16;

        let (hash, paths) = row?;
        println!("Hash: {hash}");
        println!("│");
        paths.split('|').for_each(|path| {
            line_counter += 1;
            println!(
                "├┬─ Name: {}",
                PathBuf::from(path).file_name().unwrap().to_str().unwrap(),
            );
            println!("│└─ Path: {}", path.color(BrightBlack));
            println!("│");
        });
        println!("Repetitions: {line_counter}\n");
    }
    Ok(())
}

// pub fn
