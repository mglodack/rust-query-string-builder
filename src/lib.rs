pub fn build_query(params: Vec<(&str, &str)>) -> String {
    if params.is_empty() { return String::new() }

    params.iter()
        .skip(1)
        .fold(
             _format_kvp_opt("?", params.first()),
             | mut query, kvp | {
                query.push_str(&_format_kvp_opt("&", Some(kvp)));
                query
             })
}

fn _format_kvp_opt(separator: &str, kvp: Option<&(&str, &str)>) -> String {
    match kvp {
        Some(kvp) => _format_kvp(separator, kvp),
        None => String::new()
    }
}

fn _format_kvp(separator: &str, kvp: &(&str, &str)) -> String {
    format!("{0}{1}={2}", separator, kvp.0, kvp.1)
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
    fn build_query_with_multipls_params() {
        let params = vec![("key1", "value1"), ("key2", "value2")];
        assert_eq!("?key1=value1&key2=value2", build_query(params))
    }

    #[test]
    fn empty_vector_returns_emtpy_string() {
        let params: Vec<(&str, &str)> = Vec::new();
        assert_eq!(String::new(), build_query(params))
    }
}
