fn practice(value: Option<usize>, index: usize) -> usize {
    return value.unwrap_or(index) * 5;
}

fn main() {
    let arr: Vec<Option<usize>> = vec![None, None, Some(5)];
    let result: Vec<usize> = arr.iter()
        .enumerate()
        .map(|(i, x)| practice(*x, i))
        .collect();
    println!("result :{:?}", result);
}
