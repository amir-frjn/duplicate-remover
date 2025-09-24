pub mod bfs {
    ///This mod is for breadth first search algorithm and it's implementations
    use std::{
        collections::VecDeque,
        fs::{DirEntry, read_dir},
        path::PathBuf,
        sync::mpsc::Sender,
    };

    ///takes the path that we want to start browsing from, also a channel sender to send
    ///files location to read and hash them
    pub fn bfs_search(origin_path: PathBuf, path_sender: Sender<PathBuf>) {
        //It saves directories to process them later(after files in that level)
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
                    entry_distinction(&entry, path_sender.clone(), &mut directories_list);
                }
                Err(e) => eprintln!("can't read {}: {}", path.to_str().unwrap(), e),
            });
        }
    }

    //It decides what to do with given file or directory, if is a file send it to next thread to hash it and add it
    //to directories queue to process it later
    fn entry_distinction(
        entry: &DirEntry,
        sender: Sender<PathBuf>,
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
    ///This mod is for deep first search algorithm and it's implementations
    use std::{
        fs::{DirEntry, read_dir},
        path::PathBuf,
        sync::mpsc::Sender,
    };

    ///takes the path that we want to start browsing from, also a channel sender to send
    ///files location to read and hash them
    pub fn dfs_search(origin_path: PathBuf, path_sender: Sender<PathBuf>) {
        let read_result = match read_dir(&origin_path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("can't read directory: {}", e);
                return;
            }
        };
        read_result.for_each(|f| match f {
            Ok(entry) => {
                entry_distinction(&entry, path_sender.clone());
            }
            Err(e) => eprintln!("can't read {}: {}", origin_path.to_str().unwrap(), e),
        });
    }

    fn entry_distinction(entry: &DirEntry, path_sender: Sender<PathBuf>) {
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
        println!("imported {}", entry.path().to_str().unwrap());

        path_sender.send(entry.path().to_path_buf()).unwrap();
    }
}
