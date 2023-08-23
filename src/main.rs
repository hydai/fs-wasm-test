use std::fs;

fn ls(p: &str) {
    let paths = fs::read_dir(p).unwrap();

    println!("===\nList {}\n===", p);
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

fn main() {
    ls("/");
    ls("/usr");
    ls("/usr/local");
    ls("/usr/local/lib");
}
