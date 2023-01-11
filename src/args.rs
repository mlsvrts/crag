use clap::builder::PossibleValuesParser;
use clap::Parser;

/// Search, but it's terminal
#[derive(Parser)]
pub struct Args {
    /// Query that will be sent to Custom Search API
    #[command(flatten)]
    pub query: Query,

    /// Search engine that will be used when performing query
    #[arg(short, long, default_value="qwant", value_parser=PossibleValuesParser::new(["qwant"]))]
    pub engine: String,

    /// Provide engine customization options
    #[command(flatten)]
    pub engine_options: EngineOptions,
}

/// Provides engine customization options
#[derive(clap::Args)]
pub struct EngineOptions {
    /// Do not attempt to filter ads from returned responses
    ///
    /// Note that filtering ads may impact the number of results returned
    #[arg(long)]
    pub keep_ads: bool,

    /// Do not attempt to filter video results from returned responses
    ///
    /// Note that keeping videos may result in additional results returned
    #[arg(long)]
    pub keep_videos: bool,

    /// Do not attempt to filter image results from returned responses
    ///
    /// Note filtering images may impact the number of results returned
    #[arg(long)]
    pub keep_images: bool,
}

/// Encodes custom search parameters to a query string
#[derive(clap::Args)]
pub struct Query {
    /// Query string that will be transmitted to the engine
    #[arg(trailing_var_arg = true)]
    pub query: Vec<String>,

    /// The number of responses to return when searching
    #[arg(short, long, default_value = "10")]
    pub count: u32,

    /// Set the locale when requesting the api response
    #[arg(long, default_value = "en_us")]
    pub locale: String,
}
