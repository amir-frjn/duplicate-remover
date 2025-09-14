use std::path::PathBuf;

#[derive(Debug)]
pub struct SearchOptions {
    hash_type: HashTypes,
    include_name: bool,
    include_raw: bool,
    desired_directory: PathBuf,
}
impl SearchOptions {
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
#[derive(Debug)]
pub enum HashTypes {
    SHA256,
    SHA512,
    MD5,
    BLAKE3,
    XXH3,
}
