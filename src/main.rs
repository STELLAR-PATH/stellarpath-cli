use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "StellarPath",
    version,
    about = "CLI for navigating Stellar/Soroban repositories"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a path
    Scan {
        /// The path to scan
        #[arg(default_value = ".")]
        path: String,

        /// The format of the output
        #[arg(short, long, value_enum, default_value_t = Format::Terminal)]
        format: Format,
    },
    /// Start exploring a path
    Start {
        /// The path to start exploring
        #[arg(default_value = ".")]
        path: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Terminal,
    Json,
    Markdown,
}

use std::path::PathBuf;
use stellar_path::report::{
    json::JsonRenderer, markdown::MarkdownRenderer, terminal::TerminalRenderer, ReportRenderer,
};
use stellar_path::run_scan;
use stellar_path::scanner::ScanConfig;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { path, format } => {
            let config = ScanConfig::default();
            match run_scan(&PathBuf::from(path), config) {
                Ok(result) => {
                    let renderer: Box<dyn ReportRenderer> = match format {
                        Format::Terminal => Box::new(TerminalRenderer),
                        Format::Json => Box::new(JsonRenderer),
                        Format::Markdown => Box::new(MarkdownRenderer),
                    };
                    match renderer.render(&result) {
                        Ok(output) => println!("{}", output),
                        Err(e) => {
                            eprintln!("Error rendering report: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Scan failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Start { path } => {
            let config = ScanConfig::default();
            match run_scan(&PathBuf::from(path), config) {
                Ok(result) => {
                    if result.project.recommendations.is_empty() {
                        println!("No specific starting recommendations.");
                    } else {
                        for r in &result.project.recommendations {
                            println!("Step {}: {} ({})", r.step, r.title, r.path);
                            println!("  Reason: {}", r.reason);
                            if let Some(action) = &r.suggested_action {
                                println!("  Action: {}", action);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Scan failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
