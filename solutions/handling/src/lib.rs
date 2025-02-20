use std::{fs::OpenOptions, io::Write, path::Path};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    f.write(content.as_bytes()).unwrap();
}
