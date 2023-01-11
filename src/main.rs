//! Crag
//!
//! A binary crate that provides access to search engines from the command line.

use clap::Parser;

/// Argument parsing
mod args;

/// Core search API wrapper (search over any protocol)
mod soap;

/// Implements [`soap::Soap`] for various search providers
mod engines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    let engine: Box<dyn soap::Soap> = match &args.engine[..] {
        "qwant" => Box::new(engines::Qwant::new(args.engine_options)),
        k => Err(format!("Invalid engine '{k}' was specified"))?,
    };

    let results = engine.search(args.query)?;

    for (idx, item) in results.iter().enumerate() {
        print!("\n{}. {item}\n", idx + 1);
    }

    Ok(())
}
