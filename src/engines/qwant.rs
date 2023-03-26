use crate::soap::*;

use ureq::serde_json::Value;

/// The qwant api base url
const QWANT_BASE_URL: &str = "https://api.qwant.com/v3";

/// Qwant.com search engine implementation
#[derive(Debug, Default)]
pub struct Qwant {
    options: EngineOptions,
}

impl Qwant {
    /// Build a new search API call from a query
    fn search_builder(query: Query) -> String {
        format!(
            "{QWANT_BASE_URL}/search/web?q={}&locale={}&count={}",
            query.query, query.locale, query.count
        )
    }

    /// Parse a vector of SearchResults from a JSON response
    fn parse_results(&self, resp: Value) -> Result<SearchResults, Box<dyn std::error::Error>> {
        // Get the JSON result array
        let results = resp
            .get("data")
            .and_then(|r| r.get("result"))
            .and_then(|r| r.get("items"))
            .and_then(|r| r.get("mainline"))
            .and_then(|r| r.as_array())
            .ok_or("failed to parse a result list from the qwant response")?;

        // Consume our results array:
        // This is actually an 'array of arrays', which will allow us to filter ads
        let mut output = Vec::with_capacity(10);
        for result in results {
            // Remove ads and videos from our results
            match result.get("type").and_then(|r| r.as_str()) {
                Some("ads") if !self.options.keep_ads => continue,
                Some("videos") if !self.options.keep_videos => continue,
                Some("images") if !self.options.keep_images => continue,
                _ => {}
            };

            let sub_results = result
                .get("items")
                .and_then(|r| r.as_array())
                .ok_or("encountered bad meta result")?;

            // Parse meta result groups into SearchResults
            for item in sub_results {
                // If the title or source is missing, we'll skip the result
                let title = item.get("title").and_then(|v| v.as_str());
                let source = item.get("url").and_then(|v| v.as_str());
                let description = item.get("desc").and_then(|v| v.as_str());

                if title.is_none() || source.is_none() {
                    continue;
                }

                let res = SearchResult::try_new(
                    title.unwrap_or("CRAG ERROR: BAD TITLE"),
                    description.unwrap_or("..."),
                    source.unwrap_or("CRAG_ERROR: MISSING SOURCE"),
                )?;

                output.push(res);
            }
        }

        Ok(output)
    }
}

/// Provides a search implementation for the Qwant API
impl Soap for Qwant {
    fn configure(&mut self, options: EngineOptions) -> Result<(), Box<dyn std::error::Error>> {
        self.options = options;
        Ok(())
    }

    fn search(&self, mut query: Query) -> Result<SearchResults, Box<dyn std::error::Error>> {
        let count = query.count;

        // Fix our query
        if query.count < 10 {
            query.count = 10;
        }

        // Make the initial search result request
        let resp = match ureq::get(&Qwant::search_builder(query)[..]).call() {
            Ok(r) => r,
            Err(ureq::Error::Status(code, r)) => {
                // Qwant returns an error body, so we'll print it here.
                let status_text = String::from(r.status_text());
                let body = r
                    .into_string()
                    .unwrap_or_else(|e| format!("Failed to parse body: {}", e));

                eprintln!("Error Response: {body}");

                return Err(ureq::Error::Status(
                    code,
                    ureq::Response::new(code, &status_text, &body)?,
                ))?;
            }
            Err(e) => return Err(e)?,
        };

        // Convert the response body into JSON
        let json = resp.into_json()?;

        // Parse a vector of SearchResults out of the JSON response
        let mut results = self.parse_results(json)?;

        // Truncate back down to the correct number of results
        while results.len() > count as usize {
            results.pop();
        }

        Ok(results)
    }
}
