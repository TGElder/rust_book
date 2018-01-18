use std::collections::HashMap;

struct Cacher<'a, T: 'a + Eq + std::hash::Hash, U, F> 
where F: Fn(&'a T) -> U,
{
    function: F,
    cache: HashMap<&'a T, U>,
}

impl <'a, T: Eq + std::hash::Hash, U, F> Cacher<'a, T, U, F> 
where F: Fn(&'a T) -> U,
{

    fn new(function: F) -> Cacher<'a, T, U, F> {
        Cacher{
            function,
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, value: T) -> &mut U {
        self.cache.entry(&value).or_insert((self.function)(&value))
    }
}

fn main() {
    let len_cacher = Cacher::new(|s: &String| s.len());

    println!("{}", len_cacher.get(String::from("one")));
    println!("{}", len_cacher.get(String::from("three")));
    println!("{}", len_cacher.get(String::from("one")));
}


        
