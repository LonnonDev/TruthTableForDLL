mod parser;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::parse;
    
    #[test]
    fn it_works() {
        let mut hashmappers: HashMap<String, u64> = HashMap::new();
        hashmappers.insert("A".to_owned(), 0);
        hashmappers.insert("B".to_owned(), 1);
        parse(hashmappers);
    }
}
