use clap::{Parser, Subcommand};

use crate::{rel, rel_config};

pub async fn figure() -> anyhow::Result<(String, bool)> {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        Some(Commands::Init {}) => rel_config::example(),
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        None => rel::read(),
    };

    match result {
        Ok(o) => Ok((o, cli.raw)),
        Err(err) => Err(err),
    }
}

/// dreamy cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "rel")]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    raw: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "snake_case")]
enum Commands {
    /// [STABLE] print markdown doc of relieve to std out
    Markdown,

    /// [STABLE] creates an global example config
    Init {},
}
