use std::collections::HashMap;

pub fn get_queries(path: &'_ str) -> HashMap<&'_ str, Option<&'_ str>> {
    let mut result = HashMap::new();
    let queries = match path.split_once("?") {
        Some((_, q)) => q,
        None => return result,
    };
    let queries = queries.split('&');
    for query in queries {
        if query.is_empty() {
            continue;
        }
        let (key, value) = match query.split_once("=") {
            Some(v) => v,
            None => {
                result.insert(query, None);
                continue;
            }
        };
        result.insert(key, Some(value));
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_queries() {
        assert_eq!(get_queries(""), HashMap::new());
        assert_eq!(get_queries("https://test.com"), HashMap::new());
    }

    #[test]
    fn test_one_query() {
        let mut v1 = HashMap::new();
        v1.insert("key", Some("value"));
        assert_eq!(get_queries("https://test.com?key=value"), v1);

        let mut v2 = HashMap::new();
        v2.insert("key", None);
        assert_eq!(get_queries("https://test.com?key"), v2);
    }

    #[test]
    fn test_multiple_queries() {
        let mut v1 = HashMap::new();
        v1.insert("key1", Some("value1"));
        v1.insert("key2", Some("value2"));
        assert_eq!(get_queries("https://test.com?key1=value1&key2=value2"), v1);

        let mut v2 = HashMap::new();
        v2.insert("key1", None);
        v2.insert("key2", Some("value2"));
        assert_eq!(get_queries("https://test.com?key1&key2=value2"), v2);
    }
}
