mod file;

fn main() {
    let file = file::MyFile::new("hello", "/home", 64);
    println!("{:?}", file)
}
