//! Crag
//!
//! Search from the command line

use crag::*;

use clap::builder::PossibleValuesParser;
use clap::Parser;

use colored::Colorize;

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

    /// Query string that will be transmitted to the engine
    #[arg(trailing_var_arg = true)]
    pub query_cli: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    // We need to update our query from the cli args
    args.query.query = args.query_cli.join(" ");

    let mut engine: Box<dyn Soap> = match &args.engine[..] {
        "qwant" => Box::<engines::Qwant>::default(),
        k => Err(format!("Invalid engine '{k}' was specified"))?,
    };

    engine.configure(args.engine_options)?;
    let results = engine.search(args.query)?;

    for (idx, item) in results.iter().enumerate() {
        print!(
            "\n{}. {}\n{}\n{}\n",
            idx + 1,
            item.title.green(),
            format!("{}", item.source).italic().blue(),
            item.description.dimmed().white()
        );
    }

    Ok(())
}
