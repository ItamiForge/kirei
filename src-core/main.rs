use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use kirei::{scanner, tui};

#[derive(Parser, Debug)]
#[command(name = "kirei")]
#[command(about = "MacOS system cleaner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan and show report (CLI mode)
    Scan {
        /// dry run (don't delete anything)
        #[arg(long, default_value_t = true)]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // CLI Mode: Scan and report only
        Some(Commands::Scan { dry_run }) => {
            let scanner = scanner::Scanner::new().await;
            println!("{}", "Scanning system...".to_string().cyan());
            // Perform the scan (read-only)
            let results = scanner.scan_all().await?;
            scanner::print_report(&results);
            
            // Safety guard: Ensure users know CLI deletion isn't active yet
            if !dry_run {
                println!("\nRun without --dry-run not implemented for CLI yet. Use the TUI for interactive cleaning.");
            }
        }
        // TUI Mode: Interactive interface
        None => {
            tui::run().await?;
        }
    }

    Ok(())
}
