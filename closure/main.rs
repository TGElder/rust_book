use std::collections::HashMap;

struct Cacher<T: std::hash::Hash + Eq, U, F>
    where F: Fn(T) -> U
{
    cache: HashMap<T, U>,
    function: F
}

impl <T: std::hash::Hash + Eq, U, F> Cacher<T, U, F>
    where F: Fn(T) -> U
{

    fn new(function: F) -> Cacher<T, U, F> {

        Cacher {
            cache: HashMap::new(),
            function
        }
    }

    fn get(&mut self, value: T) -> &mut U {
        self.cache.entry(value).or_insert((self.function)(value))
    }

}

fn main() {
    let mut cacher = Cacher::new(|s: &&str| { 
        println!("Computing for {}", s);
        s.len() });
    println!("{:?}", cacher.get(&"hello"));
    println!("{:?}", cacher.get(&"wtf"));
    println!("{:?}", cacher.get(&"hello"));
}

