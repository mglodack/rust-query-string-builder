pub fn build_query(params: Vec<(&str, &str)>) -> String {
    if params.is_empty() { return String::new() }

    params.iter()
        .skip(1)
        .fold(
             _initialize_query(params.first()),
             | mut query, kvp | {
                query.push_str(&format!("&{0}={1}", kvp.0, kvp.1));
                query
             })
}

fn _initialize_query(kvp: Option<&(&str, &str)>) -> String {
    match kvp {
        Some(kvp) => format!("?{0}={1}", kvp.0, kvp.1),
        None => String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_query_with_single_kvp() {
        let params = vec![("key1", "value1")];
        assert_eq!("?key1=value1", build_query(params))
    }

    #[test]
    fn it_works() {
        let params = vec![("key1", "value1"), ("key2", "value2")];
        assert_eq!("?key1=value1&key2=value2", build_query(params))
    }

    #[test]
    fn empty_vector_returns_emtpy_string() {
        let params: Vec<(&str, &str)> = Vec::new();
        assert_eq!(String::new(), build_query(params))
    }
}
