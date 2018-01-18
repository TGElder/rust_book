use std::collections::HashMap;

struct Cacher<'a, T: 'a + std::hash::Hash + Eq, U, F>
    where F: Fn(&'a T) -> U
{
    cache: HashMap<&'a T, U>,
    function: F
}

impl <'a, T: 'a + std::hash::Hash + Eq, U, F> Cacher<'a, T, U, F>
    where F: Fn(&'a T) -> U
{

    fn new(function: F) -> Cacher<'a, T, U, F> {

        Cacher {
            cache: HashMap::new(),
            function
        }
    }

    fn get(&mut self, value: &'a T) -> &mut U {
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

