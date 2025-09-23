//! # fc_generator (File cache generator)
//!
//! It provides some features to create a database(sqlite) for all files in a system including
//! their hashes(blake3, sha, xxh, md5).
//! This crate uses multithreading to speed up progress:
//!     one thread for explore inside given directory
//!     one thread to hash found files
//!     one thread to save hashes in a database
//! these threads are connected by two channels

mod explorer;
mod hasher;
mod models;
pub use crate::models::{HashTypes, SearchAlgorithm, SearchOptions};
pub use hasher::generate_cache;
