use std::rc::Rc;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(1, Rc::new(
                        Cons(2, Rc::new(
                            Cons(3, Rc::new(
                                Nil)))))));
    println!("Ref count = {}", Rc::strong_count(&a));
    let b = Cons(4, Rc::clone(&a));
    println!("Ref count = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("Ref count = {}", Rc::strong_count(&a));
    drop(b);
    println!("Ref count = {}", Rc::strong_count(&a));
    drop(c);
    println!("Ref count = {}", Rc::strong_count(&a));
    drop(a);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

use std::ops::Deref;

struct MyBox<T> (T);

impl <T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox{0: t}
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl <T> Drop for MyBox<T> {

    fn drop(&mut self) {
        println!("Dropping");
    }
}
