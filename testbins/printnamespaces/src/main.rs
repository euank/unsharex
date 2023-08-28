fn main() {
    let paths = std::fs::read_dir("/proc/self/ns").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let ns = match std::fs::read_link(&path) {
            Ok(ns) => ns,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                continue
            }
            Err(e) => {
                panic!("error reading ns link: {:?}: {}", &path, e);
            }
        };
        let fname = path.file_name().unwrap().to_owned();
        println!("{}: {}", fname.to_string_lossy(), ns.to_string_lossy());
    }
}
