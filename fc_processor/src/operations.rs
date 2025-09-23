use std::{
    fs,
    io::{Write, stdout},
};

use colored::{
    Color::{self, BrightBlack, Green, Red, Yellow},
    Colorize,
};
use rusqlite::{Connection, Result, params};
pub fn show_duplicates(db_path: &str) -> Result<()> {
    let database = Connection::open(db_path)?;
    let mut statement = database.prepare(
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
    let database = Connection::open(db_path)?;
    let mut statement = database.prepare(
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
    let mut dup_count: u32 = database.query_row(
        "SELECT COUNT(*)
       FROM files
      WHERE hash = ?1",
        params![hash],
        |row| row.get(0),
    )?;
    let mut n = 0u32;
    let mut answer = String::new();
    let stdin = std::io::stdin();
    let mut stdout = stdout();
    println!("Hash: {}", hash);
    println!("│");
    for r in rows {
        n += 1;
        let (path, name) = r?;
        println!("│ {n} ─┬─ name: {name}");
        println!("│    └─ path: {}", path.color(BrightBlack));
        if dup_count == 1 {
            println!("│{}", "Warning: This file is now unique.".color(Yellow));
        }
        print!("│ Remove ? ({}, {}) ", "N".color(Green), "y".color(Red));
        stdout.flush().unwrap();
        answer.clear();
        stdin.read_line(&mut answer).unwrap();

        if answer.trim() != "y" && answer != "Y" {
            println!("│ {}", "Kept".color(Color::Green));
            println!("│");
            continue;
        }
        if let Err(e) = fs::remove_file(&path) {
            eprintln!("│ Couldn't remove: {}", e);
        } else if let Err(e) = database.execute("DELETE FROM files WHERE path = ?1", params![path])
        {
            eprintln!(
                "│ Couldn't remove it from database(regenerate it later) : {}",
                e
            );
        }
        dup_count -= 1;
        if dup_count == 0 {
            print!("└");
        } else {
            print!("│");
        }
        println!("{}", " Removed".color(Color::Red));
        println!("│");
    }
    if n < 2 {
        eprintln!("No duplicate files were found with the given hash!");
        return Ok(());
    }
    print!("{}: {}", "Removed".color(Red), n - dup_count,);
    println!(" , {}: {}", "Kept".color(Green), n);
    println!("");
    Ok(())
}
pub fn remove_all(db_path: &str) -> Result<()> {
    let database = Connection::open(db_path)?;
    let mut statement = database.prepare(
        "SELECT hash
       FROM files
      GROUP BY hash
     HAVING COUNT(*) > 1",
    )?;

    let hashes: Vec<Result<String, rusqlite::Error>> = statement
        .query_map([], |row| row.get::<_, String>(0))?
        .collect();

    for hash_result in hashes {
        let hash = hash_result?;
        remove_by_hash(db_path, &hash)?;
    }
    Ok(())
}
// pub fn
