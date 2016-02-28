# rust-query-string-builder

[![Build Status](https://travis-ci.org/mglodack/rust-query-string-builder.svg?branch=master)](https://travis-ci.org/mglodack/rust-query-string-builder)

#### How to use

```rust
extern crate query_string_builder;

using query_string_builder::*;

static AUTH_URL: &'static str = "https://accounts.somewebpage.com/authorize/";

fn create_auth_query_string() -> String {
  let params = vec!
    [
      ("client_id", "123456"),
      ("client_secret", "secret")
    ];
    
  build_query_string(params)
}

fn create_auth_url_string() -> String {
  build_url_string(AUTH_URL, &create_auth_query_string())
}
```
