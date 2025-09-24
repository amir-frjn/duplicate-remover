use crate::explorer::{bfs, dfs};
use crate::models::{HashBuffer, HashDetails, HashTypes, SearchAlgorithm, SearchOptions};
use mpsc::channel;
use std::{
    error::Error,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self},
};

///Takes a SearchOptions object and creates a database including files and their hashes
pub fn generate_cache(search_options: SearchOptions) -> Result<(), Box<dyn std::error::Error>> {
    let origin_path = search_options.get_origin().clone();
    let hash_type = search_options.get_hash_type().clone();
    let search = match search_options.get_search_type() {
        SearchAlgorithm::BFS => bfs::bfs_search,
        SearchAlgorithm::DFS => dfs::dfs_search,
    };

    //It connects search and read_raw_binaries functions
    let (path_sender, path_receiver) = channel();

    //It connects read_raw_binaries and save_hashes functions
    let (hash_detail_sender, hash_detail_receiver) = channel();

    //join handles
    let search_handle = thread::spawn(move || search(origin_path, path_sender));
    let read_file_handle =
        thread::spawn(|| read_raw_binaries(path_receiver, hash_type, hash_detail_sender));
    let save_hashes_handle = thread::spawn(|| {
        if let Err(e) = save_hashes(hash_detail_receiver) {
            eprintln!("can't create database: {}", e);
        }
    });

    if let Err(e) = search_handle.join() {
        return Err(format!("Search thread panicked: {:?}", e).into());
    }
    if let Err(e) = read_file_handle.join() {
        return Err(format!("Read file thread panicked: {:?}", e).into());
    }
    if let Err(e) = save_hashes_handle.join() {
        return Err(format!("Save hashes thread panicked: {:?}", e).into());
    }
    Ok(())
}

///Takes file paths from the explorer's channel,
///generates checksum hashes according to hash_type, and forwards them to a channel for database saving.
fn read_raw_binaries(
    path_receiver: Receiver<PathBuf>,
    hash_type: HashTypes,
    hash_detail_sender: Sender<HashDetails>,
) {
    for path in path_receiver {
        let hash_buffer = match HashBuffer::new(path) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("can't read {}", e);
                continue;
            }
        };
        let hash = match hash_type {
            HashTypes::BLAKE3 => hash_buffer.blake3_hash(),
            HashTypes::MD5 => hash_buffer.md5_hash(),
            HashTypes::SHA1 => hash_buffer.sha1_hash(),
            HashTypes::XXH3 => hash_buffer.xxh3_hash(),
            HashTypes::SHA256 => hash_buffer.sha256_hash(),
        };
        hash_detail_sender.send(hash).unwrap();
    }
}

use rusqlite::{Connection, params};
///Receives a file path and it's hash, then save it to a database
fn save_hashes(hash_detail_receiver: Receiver<HashDetails>) -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open("files.db")?;

    //create an empty database with rows including (path, name, hash)
    conn.execute("DROP TABLE IF EXISTS files", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL,
            name TEXT NOT NULL,
            hash TEXT NOT NULL
        )",
        [],
    )?;

    let tx = conn.transaction()?;

    //push data to  database
    for hash_details in hash_detail_receiver {
        let path = hash_details.path().to_str();
        let name = hash_details.name();
        let hash = hash_details.hash();
        let result = tx.execute(
            "INSERT INTO files (path, name, hash) VALUES (?1, ?2, ?3)",
            params![path, name, hash],
        );
        if let Err(e) = result {
            eprintln!("can't write to database: {}", e);
            continue;
        }
        println!("imported: {}", path.unwrap());
    }

    //remove unique files from database
    tx.execute(
        "DELETE FROM files
         WHERE hash IN (
             SELECT hash
             FROM files
             GROUP BY hash
             HAVING COUNT(*) = 1
         )",
        [],
    )?;
    tx.commit()?;
    Ok(())
}
