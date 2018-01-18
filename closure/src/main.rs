use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Cacher<T: std::hash::Hash + Eq + Copy, U, F>
    where F: Fn(T) -> U
{
    cache: HashMap<T, U>,
    function: F
}

impl <T: std::hash::Hash + Eq + Copy, U, F> Cacher<T, U, F>
    where F: Fn(T) -> U
{

    fn new(function: F) -> Cacher<T, U, F> {

        Cacher {
            cache: HashMap::new(),
            function
        }
    }

    fn get(&mut self, value: T) -> &mut U {
        match self.cache.entry(value) {
            Entry::Vacant(v) => v.insert((self.function)(value)),
            Entry::Occupied(v) => v.into_mut()
        }
    }

}

fn main() {
    let mut cacher = Cacher::new(|s: &str| { 
        println!("Computing for {}", s);
        s.len() });
    println!("{:?}", cacher.get("hello"));
    println!("{:?}", cacher.get("wtf"));
    println!("{:?}", cacher.get("hello"));
}

