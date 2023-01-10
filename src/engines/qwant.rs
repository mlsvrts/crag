use crate::soap::*;

/// The qwant api base url
const QWANT_BASE_URL: &str = "https://api.qwant.com/v3";

#[derive(Clone)]
pub struct Qwant;

impl Qwant {
    fn search_builder(query: Query) -> String {
        format!(
            "{QWANT_BASE_URL}/search/web?q={}&locale={}&count={}",
            query.query,
            query.locale,
            query.count
        )
    }
}

impl Soap for Qwant { 
    fn search(&self, query: Query) -> Result<SearchResults, Box<dyn std::error::Error>> {
        let resp = match ureq::get(&Qwant::search_builder(query)[..]).call() {
            Ok(r) => r,
            Err(ureq::Error::Status(code, r)) => {
                // Qwant returns an error body, so we'll print it here.
                let status_text = String::from(r.status_text());
                let body = r.into_string().unwrap_or("Failed".into());

                eprintln!("Error Response: {body}");

                return Err(ureq::Error::Status(code, ureq::Response::new(code, &status_text, &body)?))?;
            },
            Err(e) => return Err(e)?
        };

        return Ok(resp.into_string()?);
    }
}
