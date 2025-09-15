use std::{
    fs::File,
    hash::Hasher,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

#[derive(Debug)]
pub struct SearchOptions {
    hash_type: HashTypes,
    include_name: bool,
    include_raw: bool,
    desired_directory: PathBuf,
}
impl SearchOptions {
    pub fn hash_typd(&self) -> HashTypes {
        self.hash_type.clone()
    }
    pub fn new() -> Self {
        Self {
            hash_type: HashTypes::BLAKE3,
            include_name: true,
            include_raw: true,
            desired_directory: "./".into(),
        }
    }
    pub fn get_path(&self) -> PathBuf {
        self.desired_directory.clone()
    }
    pub fn set_origin(&mut self, parent_path: &str) {
        self.desired_directory = parent_path.into();
    }
    pub fn include_name(&mut self, name_stat: bool) {
        self.include_name = name_stat;
    }
    pub fn include_raw(&mut self, raw_stat: bool) {
        self.include_raw = raw_stat;
    }

    pub fn hash_type(&mut self, hash_type: HashTypes) {
        self.hash_type = hash_type;
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
    buffer: [u8; 2_000],
}

impl HashBuffer {
    pub fn new(file_handle: File) -> HashBuffer {
        HashBuffer {
            buff_reader: BufReader::new(file_handle),
            buffer: [0; 2_000],
        }
    }

    fn buffer_data(&mut self) -> Option<usize> {
        let n = self.buff_reader.read(&mut self.buffer).unwrap();
        if n > 0 {
            return Some(n);
        }
        return None;
    }
    pub fn blake3_hash(&mut self) -> String {
        let mut hasher = blake3::Hasher::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        hasher.finalize().to_string()
    }
    pub fn xxh3_hash(&mut self) -> String {
        let mut hasher = Xxh3::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        format!("{:x}", hasher.digest128())
    }
    pub fn md5_hash(&mut self) -> String {
        let mut hasher = Md5::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    pub fn sha1_hash(&mut self) -> String {
        let mut hasher = sha1::Sha1::new();
        while let Some(n) = self.buffer_data() {
            hasher.update(&self.buffer[..n]);
        }
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
