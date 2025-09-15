pub mod bfs {
    use std::{
        collections::VecDeque,
        fs::{DirEntry, read_dir},
        path::PathBuf,
        sync::mpsc::Sender,
    };
    pub fn bfs_search(origin_path: PathBuf, sender: Sender<DirEntry>) {
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
                    entry_distinction(entry, &sender, &mut directories_list);
                    println!("imported {}", path.to_str().unwrap());
                }
                Err(e) => eprintln!("can't read {}: {}", path.to_str().unwrap(), e),
            });
        }
    }
    fn entry_distinction(
        entry: DirEntry,
        sender: &Sender<DirEntry>,
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

        sender.send(entry).unwrap();
    }
}
pub mod dfs {
    use std::{
        fs::{DirEntry, read_dir},
        path::PathBuf,
        sync::mpsc::Sender,
    };

    pub fn dfs_search(origin_path: PathBuf, sender: &Sender<DirEntry>) {
        let read_result = match read_dir(&origin_path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("can't read directory: {}", e);
                return;
            }
        };
        read_result.for_each(|f| match f {
            Ok(entry) => {
                entry_distinction(entry, sender);
                println!("imported {}", origin_path.to_str().unwrap());
            }
            Err(e) => eprintln!("can't read {}: {}", origin_path.to_str().unwrap(), e),
        });
    }
    fn entry_distinction(entry: DirEntry, sender: &Sender<DirEntry>) {
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(e) => {
                eprintln!("can't get file type: {e}");
                return;
            }
        };
        if file_type.is_dir() {
            dfs_search(entry.path(), sender);
            return;
        }
        sender.send(entry).unwrap();
    }
}
