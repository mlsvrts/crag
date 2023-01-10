use clap::Parser;
use clap::builder::ValueParser;

use crate::soap::Soap;

// Enables support for deriving 'Clone'
dyn_clone::clone_trait_object!(Soap); 

/// Google, for people who are allergic to GUIs
#[derive(Parser)]
pub struct Args {
    /// Query that will be sent to Custom Search API
    #[command(flatten)]
    pub query: Query,

    /// Search engine that will be used when performing query
    ///
    /// Must be one of [qwant]
    #[arg(value_parser=ValueParser::new(engine_parse), default_value="qwant")]
    pub engine: Box<dyn Soap>,
}

/// Encodes custom search parameters to a query string
#[derive(clap::Args)]
pub struct Query {    
    /// Query string that will be transmitted to the engine
    pub query: String,

    /// The number of responses to return when searching
    #[arg(short, long, default_value="10")]
    pub count: u32,

    /// Set the locale when requesting the api response
    #[arg(long, default_value="en_us")]
    pub locale: String,
}

fn engine_parse(value: &str) -> Result<Box<dyn Soap>, String> {
    match value {
        "qwant" => Ok(Box::new(crate::engines::Qwant {})),
        engine => Err(format!("'{engine}' is not a valid search provider. Must be one of [qwant]"))
    }
}
