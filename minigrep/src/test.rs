#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duck";
        let text = "I wish
            I had
            duck feet";
        assert_eq!(search(query, text), vec!["duck feet"]);

}
