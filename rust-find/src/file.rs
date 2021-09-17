mod file;

#[derive(Debug)]
pub struct MyFile {
    name: String,
    dir_in: String,
    size_bytes: u64,
}

impl MyFile {
    pub fn new(name: &str, dir: &str, size: u64) -> MyFile {
        MyFile{
            name: name.to_string(),
            dir_in: dir.to_string(),
            size_bytes: size
        }
    }
}
