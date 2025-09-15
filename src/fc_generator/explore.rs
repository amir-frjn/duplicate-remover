pub mod bfs {
    use std::{
        collections::VecDeque,
        fs::{DirEntry, read_dir},
        path::PathBuf,
        sync::mpsc::Sender,
    };
    pub fn bfs_search(origin_path: PathBuf, path_sender: Sender<PathBuf>) {
        let mut directories_list = VecDeque::with_capacity(30);

        directories_list.push_back(origin_path);

        while let Some(path) = directories_list.pop_front() {
            let read_result = match read_dir(&path) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("can't read directory: {}", e);
                    continue;
                }
            };

            read_result.for_each(|f| match f {
                Ok(entry) => {
                    entry_distinction(entry, &path_sender, &mut directories_list);
                    println!("imported {}", path.to_str().unwrap());
                }
                Err(e) => eprintln!("can't read {}: {}", path.to_str().unwrap(), e),
            });
        }
    }
    fn entry_distinction(
        entry: DirEntry,
        sender: &Sender<PathBuf>,
        directories_list: &mut VecDeque<PathBuf>,
    ) {
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(e) => {
                eprintln!("can't get file type: {e}");
                return;
            }
        };

        if file_type.is_dir() {
            directories_list.push_back(entry.path());
            return;
        }

        sender.send(entry.path().to_path_buf()).unwrap();
    }
}
pub mod dfs {
    use std::{
        fs::{DirEntry, read_dir},
        path::PathBuf,
        sync::mpsc::Sender,
    };

    pub fn dfs_search(origin_path: PathBuf, path_sender: &Sender<PathBuf>) {
        let read_result = match read_dir(&origin_path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("can't read directory: {}", e);
                return;
            }
        };
        read_result.for_each(|f| match f {
            Ok(entry) => {
                entry_distinction(entry, path_sender);
                println!("imported {}", origin_path.to_str().unwrap());
            }
            Err(e) => eprintln!("can't read {}: {}", origin_path.to_str().unwrap(), e),
        });
    }
    fn entry_distinction(entry: DirEntry, path_sender: &Sender<PathBuf>) {
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(e) => {
                eprintln!("can't get file type: {e}");
                return;
            }
        };
        if file_type.is_dir() {
            dfs_search(entry.path(), path_sender);
            return;
        }

        path_sender.send(entry.path().to_path_buf()).unwrap();
    }
}
