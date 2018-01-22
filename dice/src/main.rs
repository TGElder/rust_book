fn main() {
    let sides = vec![1, 2, 3, 4, 5, 6];
    let out: Vec<i32> = sides.iter().zip(sides.iter())
             .map(|(a, b)| a + b)
             .filter(|s| *s == 7)
             .collect();
    println!("{:?}", out);
}
