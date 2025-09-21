use std::{
    error::Error,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self},
};

use crate::fc_generator::models::{
    HashBuffer, HashDetails, HashTypes, SearchAlgorithm, SearchOptions,
};

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
use crate::fc_generator::explore::{bfs, dfs};
use mpsc::channel;
use rusqlite::{Connection, params};
fn save_hashes(hash_detail_receiver: Receiver<HashDetails>) -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open("files.db")?;

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
        }
    }

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
pub fn generate_cache(search_options: SearchOptions) -> Result<(), Box<dyn std::error::Error>> {
    let origin_path = search_options.get_origin().clone();
    let hash_type = search_options.get_hash_type().clone();
    let search = match search_options.get_search_type() {
        SearchAlgorithm::BFS => bfs::bfs_search,
        SearchAlgorithm::DFS => dfs::dfs_search,
    };

    let (path_sender, path_receiver) = channel();
    let (hash_detail_sender, hash_detail_receiver) = channel();
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
