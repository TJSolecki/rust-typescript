fn main() {
    println!("Hello, world!");
    let list: Vec<_> = vec![1, 2, 3]
        .iter()
        .map(|x| x+1 )
        .collect();
    println!("{:?}", list);

    let list_to_collect = vec![4, 5, 6];
    let mut list_iter = list_to_collect
        .iter()
        .map(|x| x + 1);

    let mut new_vector = vec![];
    while let Some(x) = list_iter.next() {
        new_vector.push(x);
    }
    println!("{:?}", new_vector);
}
