use std::{
    collections::VecDeque,
    error::Error,
    fs::{DirEntry, read_dir},
    path::PathBuf,
    rc::Rc,
    sync::mpsc::Sender,
};
pub fn bfs_search(origin_path: PathBuf, sender: Sender<DirEntry>)  {
    let sender = Rc::from(sender);
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
                entry_distinction(entry, sender.clone(), &mut directories_list);
                println!("imported {}", path.to_str().unwrap());
            }
            Err(e) => eprintln!("can't read {}: {}", path.to_str().unwrap(), e),
        });
    }
}

fn dfs_search(origin_path: PathBuf, sender: Sender<DirEntry)
fn entry_distinction(
    entry: DirEntry,
    sender: Rc<Sender<DirEntry>>,
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

