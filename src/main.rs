fn foo(value: Option<usize>) -> Option<usize> {
    // return value.map(|x| x * 5);
    return Some(value? * 5);
}

fn main() {
    let value: Option<usize> = foo(Some(5));
    println!("{:?}", value);
    let value: Option<usize> = foo(None);
    println!("{:?}", value);
}
