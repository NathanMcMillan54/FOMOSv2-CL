fn main() {
    let paths = fs::read_dir("commands/pwdFail").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}