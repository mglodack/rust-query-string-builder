pub fn build_url_string(url: &str, query_string: &str) -> String {
    format!("{0}{1}", url, query_string)
}

pub fn build_query_string(params: Vec<(&str, &str)>) -> String {
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

    static  URL: &'static str = "http://weirddomain.com/";

    #[test]
    fn build_query_with_single_kvp() {
        let params = vec![("key1", "value1")];
        assert_eq!("?key1=value1", build_query_string(params))
    }

    #[test]
    fn build_query_with_multiple_params() {
        let params = vec![("key1", "value1"), ("key2", "value2")];
        assert_eq!("?key1=value1&key2=value2", build_query_string(params))
    }

    #[test]
    fn empty_vector_returns_emtpy_string() {
        let params: Vec<(&str, &str)> = Vec::new();
        assert_eq!(String::new(), build_query_string(params))
    }

    #[test]
    fn build_url_string_with_multiple_params()
    {
        let params = vec![("user_id", "1"), ("apple_no", "8")];

        let query_string = build_query_string(params);
        assert_eq!(
            "http://weirddomain.com/?user_id=1&apple_no=8",
            build_url_string(URL, &query_string))
    }

    #[test]
    fn build_url_string_with_emtpy_params()
    {
        let params: Vec<(&str, &str)> = Vec::new();
        let query_string = build_query_string(params);

        assert_eq!(
            URL,
            build_url_string(URL, &query_string))
    }
}
