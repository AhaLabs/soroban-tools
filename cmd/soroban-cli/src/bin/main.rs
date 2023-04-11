use clap::{CommandFactory, Parser};
use soroban_cli::{commands::plugin, Root};

#[tokio::main]
async fn main() {
    let root = Root::try_parse().unwrap_or_else(|e| {
        if let clap::error::ErrorKind::InvalidSubcommand = e.kind() {
            if let Err(error) = plugin::run() {
                eprintln!("error: {error}");
                std::process::exit(1)
            }
            e.exit()
        } else {
            let mut cmd = Root::command();
            e.format(&mut cmd).exit();
        }
    });

    if let Err(e) = root.run().await {
        eprintln!("error: {e}");
    }
}
