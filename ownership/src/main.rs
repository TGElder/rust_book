fn main() {
    let mut s = String::from("Whathaburt");

    {
        let r1 = &mut s;
        r1.push_str(" the burtiest");
    }

    let r2 = &mut s;
    r2.push_str(" burt that ever burted");
    
    println!("{}", r2);
}
