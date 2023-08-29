fn main() {
    let file_name = std::env::args().nth(1)
        .expect("the file name should be passed in");

    let file = std::fs::read_to_string(file_name)
        .expect("unable to read the provided file to string");

    file.lines().for_each(|line| println!("{}", line));
}
