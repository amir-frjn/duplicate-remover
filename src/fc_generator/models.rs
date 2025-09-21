use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
pub struct HashDetails {
    file_path: PathBuf,
    hash: String,
}
impl HashDetails {
    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn name(&self) -> &str {
        self.file_path.file_name().unwrap().to_str().unwrap()
    }
    pub fn hash(&self) -> &str {
        &self.hash
    }
}
#[derive(Debug)]
pub enum SearchAlgorithm {
    BFS,
    DFS,
}
pub struct SearchOptions {
    hash_type: HashTypes,
    desired_directory: PathBuf,
    search_algorithm: SearchAlgorithm,
}

impl SearchOptions {
    pub fn get_search_type(&self) -> &SearchAlgorithm {
        &self.search_algorithm
    }
    pub fn get_hash_type(&self) -> &HashTypes {
        &self.hash_type
    }
    pub fn set_search(mut self, search_algorithm: SearchAlgorithm) -> Self {
        self.search_algorithm = search_algorithm;
        self
    }
    pub fn new() -> Self {
        Self {
            hash_type: HashTypes::BLAKE3,
            desired_directory: "./".into(),
            search_algorithm: SearchAlgorithm::BFS,
        }
    }
    pub fn get_origin(&self) -> &PathBuf {
        &self.desired_directory
    }
    pub fn set_origin(mut self, origin_path: &str) -> Self {
        self.desired_directory = origin_path.into();
        self
    }

    pub fn set_hash_type(mut self, hash_type: HashTypes) -> Self {
        self.hash_type = hash_type;
        self
    }
}
#[derive(Debug, Clone)]
pub enum HashTypes {
    SHA256,
    SHA1,
    MD5,
    BLAKE3,
    XXH3,
}

use blake3;
use md5::{Digest, Md5};
use sha1::Sha1;
use xxhash_rust::xxh3::Xxh3;
pub struct HashBuffer {
    buff_reader: BufReader<File>,
    buffer: [u8; 20_000],
    file_path: PathBuf,
}

impl HashBuffer {
    pub fn new(file_path: PathBuf) -> Result<HashBuffer, Box<dyn std::error::Error>> {
        let file_handle = File::open(&file_path)?;

        let buff_reader = BufReader::new(file_handle);
        Ok(HashBuffer {
            buff_reader,
            buffer: [0; 20_000],
            file_path,
        })
    }

    fn buffer_data(&mut self) -> Option<usize> {
        let n = self.buff_reader.read(&mut self.buffer).unwrap();
        if n > 0 {
            return Some(n);
        }
        return None;
    }
    pub fn blake3_hash(mut self) -> HashDetails {
        let mut hasher = blake3::Hasher::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        HashDetails {
            file_path: self.file_path,
            hash: hasher.finalize().to_string(),
        }
    }
    pub fn xxh3_hash(mut self) -> HashDetails {
        let mut hasher = Xxh3::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        HashDetails {
            file_path: self.file_path,
            hash: hasher.digest128().to_string(),
        }
    }
    pub fn md5_hash(mut self) -> HashDetails {
        let mut hasher = Md5::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        HashDetails {
            file_path: self.file_path,
            hash: format!("{:x}", hasher.finalize()),
        }
    }
    pub fn sha1_hash(mut self) -> HashDetails {
        let mut hasher = Sha1::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        HashDetails {
            file_path: self.file_path,
            hash: format!("{:x}", hasher.finalize()),
        }
    }
    pub fn sha256_hash(mut self) -> HashDetails {
        let mut hasher = sha2::Sha256::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        HashDetails {
            file_path: self.file_path,
            hash: format!("{:x}", hasher.finalize()),
        }
    }
}
