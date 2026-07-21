use clap::Parser;
use day::cli::{run, unavailable, Cli};

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            unavailable()
        }
    }
}
