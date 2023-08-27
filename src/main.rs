fn main() {
    let file = std::fs::read_to_string("text.txt").unwrap();

    file.lines()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}
