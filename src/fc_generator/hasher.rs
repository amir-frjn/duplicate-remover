use std::{
    fs::{File, OpenOptions},
    path::{self, PathBuf},
    string,
    sync::mpsc::{Receiver, Sender},
};

fn read_raw_binaries(path_receiver: Receiver<PathBuf>, open_options: &OpenOptions) {
    for path in path_receiver {
        let file_name = path.file_name().unwrap();
        let file_handle = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("can't read {}: {}", file_name.to_str().unwrap(), e);
                continue;
            }
        };
        
    }
}
fn hash_raw_binaries(dir_entry_sender: Sender<(String, String)>, open_options: &OpenOptions) {}
