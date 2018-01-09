use std::collections::HashMap;

fn average(numbers: &[f32]) -> (f32, f32, i32) {
    let mean = numbers.iter().fold(0.0, |sum, x| sum + x) / (numbers.len() as f32);
    let median = numbers[numbers.len() / 2]; 
    
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for n in numbers {
        *(counts.entry(*n as i32).or_insert(0)) += 1;
    }

    println!("{:?}", counts);

    let mode = *(counts.iter().fold(None, |argmax, x| {
        match argmax {
            None => Some(x),
            Some(t) => if x.1 > t.1 {Some(x)} else {argmax},
        }
    }
    ).unwrap().0);

    (mean, median, mode)
}

fn main() {
    let numbers = vec![4.0, 8.0, 15.0, 16.0, 23.0, 8.0];
    println!("The averages are {:?}", average(&numbers));
}
