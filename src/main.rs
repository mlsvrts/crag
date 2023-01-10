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

    let results = args.engine.search(args.query)?;

    println!("{results}");

    Ok(())
}
