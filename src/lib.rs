pub fn build_query(params: Vec<(&str, &str)>) -> String {
    if params.is_empty() { return String::new() }

    let first = _assign_single_kvp(params.first());
    let last = _assign_multiple_kvp(params);

    format!("?{0}{1}", first, last)
}

fn _assign_single_kvp(kvp: Option<&(&str, &str)>) -> String {
    match kvp {
        Some(kvp) => format!("{0}={1}", kvp.0, kvp.1),
        None => String::new()
    }
}

fn _assign_multiple_kvp(params: Vec<(&str, &str)>) -> String {
    params.iter()
        .skip(1)
        .fold(
            String::new(),
            | mut query, kvp | {
                query.push_str(&format!("&{0}={1}", kvp.0, kvp.1));
                query
            })
}

#[cfg(test)]
mod tests {
    use super::*;

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
