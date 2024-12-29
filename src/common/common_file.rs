use std::fs::File;
use std::io::Read;
use futures::channel::mpsc::{self, Receiver};
use notify::{Watcher, RecursiveMode}; // 添加 notify 库的导入

pub fn common_open_file(file: &str) -> Option<String> {
    // 打开JSON文件
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(_) => return None,
    };
    // 读取文件内容 
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return None;
    }
    return Some(contents)
}

pub fn common_watch_file(file: &str) -> Receiver<()> {
    let (tx, rx) = mpsc::channel::<()>(32);
    let mut watcher = notify::recommended_watcher(move |_| {
        let _ = tx.clone().try_send(());
    }).unwrap();
    watcher.watch(file.as_ref(), RecursiveMode::NonRecursive).unwrap();
    return rx
}
