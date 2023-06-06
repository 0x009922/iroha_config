use clap::Parser;
use color_eyre::{eyre::WrapErr, Result};
use schematic::ConfigLoader;
use std::path::PathBuf;

mod config;
mod util;

#[derive(Parser)]
struct Cli {
    /// Path to the TOML or JSON config file
    path: PathBuf,
}

fn main() -> Result<()> {
    let Cli { path: config_path } = Cli::parse();

    let config: config::Iroha = ConfigLoader::<config::Iroha>::new()
        .file(config_path)
        .wrap_err("invalid file source")?
        .load()
        .wrap_err("invalid configuration!")?
        .config;

    let toml_back = toml::to_string_pretty(&config).wrap_err("failed to convert to TOML")?;
    println!("{toml_back}");

    Ok(())
}
