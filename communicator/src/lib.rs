pub mod client; 

pub mod network; 

#[cfg(test)]
mod tests {
    use client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
