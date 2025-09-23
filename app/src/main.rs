//! # app
//!
//! connects both fc_generator and fc_processor as a command line interface package
use clap::{Arg, ArgMatches, Command};
use fc_generator::{HashTypes, SearchAlgorithm, SearchOptions};
fn main() {
    let matches = clap::Command::new("rmdup")
        .about("Detects and Removes duplicate files")
        .subcommand(
            Command::new("gen-cache")
                .arg(
                    Arg::new("origin")
                        .short('p')
                        .long("path")
                        .default_value("./")
                        .help("Path to start exploration"),
                )
                .arg(
                    Arg::new("hash type")
                        .long("set-hash")
                        .default_value("blake3")
                        .help("Set hashing algorithm (sha256, sha1, md5, blake3, xxh3)"),
                )
                .arg(
                    Arg::new("search type")
                        .long("set-search")
                        .default_value("BFS")
                        .help("Set searching algorithm (BFS, DFS)"),
                )
                .about("Generates a cache for duplicate files"),
        )
        .subcommand(Command::new("show").about("Shows duplicate files (Generate cache first)"))
        .subcommand(
            Command::new("rm")
                .arg(
                    Arg::new("by hash")
                        .long("hash")
                        .num_args(1)
                        .help("Remove files using a specific hash"),
                )
                .about("Removes duplicate files"),
        )
        .subcommand_required(true)
        .get_matches();

    match matches.subcommand() {
        Some(("gen-cache", arg)) => generate_cache(arg),
        Some(("show", _)) => show_duplicates(),
        Some(("rm", arg)) => remove_duplicates(arg),
        _ => (),
    }
}

use fc_processor::operations;
fn remove_duplicates(arg: &ArgMatches) {
    let hash = arg.get_one::<String>("by hash");
    if hash.is_none() {
        if let Err(e) = operations::remove_all("files.db") {
            eprintln!("An error occurred: {e}");
            return;
        }
    } else if let Err(e) = operations::remove_by_hash("files.db", hash.unwrap()) {
        eprintln!("An error occurred: {e}");
    };
}
fn show_duplicates() {
    if let Err(e) = operations::show_duplicates("files.db") {
        eprintln!("An error occurred: {e}");
    };
}

use fc_generator::generate_cache as generate_cache_core;
fn generate_cache(arg: &ArgMatches) {
    let origin_path = arg.get_one::<String>("origin").unwrap();
    let hash_type = arg.get_one::<String>("hash type").unwrap().to_lowercase();
    let hash_type = match hash_type.as_str() {
        "blake3" => HashTypes::BLAKE3,
        "sha256" => HashTypes::SHA256,
        "sha1" => HashTypes::SHA1,
        "xxh3" => HashTypes::XXH3,
        "md5" => HashTypes::MD5,
        _ => {
            eprintln!("Invalid hashing algorithm.");
            return;
        }
    };
    let search_algorithm = match arg
        .get_one::<String>("search type")
        .unwrap()
        .to_lowercase()
        .as_str()
    {
        "bfs" => SearchAlgorithm::BFS,
        "dfs" => SearchAlgorithm::DFS,
        _ => {
            eprintln!("Invalid searching algorithm.");
            return;
        }
    };
    let search_options = SearchOptions::new()
        .set_hash_type(hash_type)
        .set_origin(origin_path)
        .set_search(search_algorithm);
    if let Err(e) = generate_cache_core(search_options) {
        eprintln!("An error occurred: {}", e);
    }
}
