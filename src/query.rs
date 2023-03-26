/// Encodes custom search parameters to a query string
#[derive(Debug, Default)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct Query {
    /// Query string that will be transmitted to the engine
    #[cfg_attr(feature = "cli", arg(skip))]
    pub query: String,

    /// The number of responses to return when searching
    #[cfg_attr(feature = "cli", arg(short, long, default_value = "10"))]
    pub count: u32,

    /// Set the locale when requesting the api response
    #[cfg_attr(feature = "cli", arg(long, default_value = "en_us"))]
    pub locale: String,
}

/// Implement From<Into<String>> for Query
///
/// Uses a default count of 10 and en_us locale
impl<T: Into<String>> From<T> for Query {
    fn from(query: T) -> Self {
        Self {
            query: query.into(),
            count: 10,
            locale: "en_us".to_string(),
        }
    }
}

impl Query {
    /// Build a new Query from a query string, count, and locale
    pub fn new<Q: Into<String>, L: Into<String>>(query: Q, count: u32, locale: L) -> Self {
        Self {
            query: query.into(),
            count,
            locale: locale.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_new() {
        // new from a basic string
        let query = Query::new("test", 10, "en_us");
        assert_eq!(query.query, "test");
    }

    #[test]
    fn from_string() {
        // from a basic string
        let query: Query = "test".into();
        assert_eq!(query.query, "test");
    }
}
