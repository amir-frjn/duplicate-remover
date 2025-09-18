use colored::{Color::BrightBlack, Colorize};
use rusqlite::{Connection, Result, params};
pub fn show_duplicates(db_path: &str) -> Result<()> {
    let connection = Connection::open(db_path)?;
    let mut statement = connection.prepare(
        "SELECT hash,
            GROUP_CONCAT(path, '|'),
            GROUP_CONCAT(name, '|')
         FROM files
         GROUP BY hash
         HAVING COUNT(*) > 1",
    )?;
    for row in statement.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
        ))
    })? {
        let mut line_counter = 0u16;

        let (hash, paths, names) = row?;
        println!("Hash: {hash}");
        println!("│");
        let mut name = names.split('|');
        paths.split('|').for_each(|path| {
            line_counter += 1;
            println!("├┬─ Name: {}", name.next().unwrap(),);
            println!("│└─ Path: {}", path.color(BrightBlack));
            println!("│");
        });
        println!("Repetitions: {line_counter}\n\n");
    }
    Ok(())
}
pub fn remove_by_hash(db_path: &str, hash: &str) -> Result<()> {
    let connection = Connection::open(db_path)?;
    let mut statement = connection.prepare(
        "SELECT path, name
     FROM files
     WHERE hash = ?1
       AND hash IN (
           SELECT hash
           FROM files
           GROUP BY hash
           HAVING COUNT(*) > 1
       )",
    )?;
    let rows = statement.query_map(params![hash], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut n = 0u16;
    for r in rows {
        n += 1;
        let (path, name) = r?;
        println!("{n} ─┬─ {name}");
        println!("   └─ {}\n", path.color(BrightBlack));
    }
    if n < 2 {
        println!("No duplicate files were found with the given hash!");
        return Ok(());
    }

    Ok(())
}

// pub fn
