fn main() {
    let a = vec![1, 2, 3, 4];

    let b: Vec<_> = a.iter().map(|e| {e * 2}).collect();

    println!("The value is {:?}", b)
}
