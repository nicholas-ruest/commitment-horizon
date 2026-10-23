use ch_errors::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Seed {
        #[arg(default_value = "payments-slo")]
        id: String,
    },
    Ablate,
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Seed { id } => println!(
            "{}",
            serde_json::to_string_pretty(&ch_seed::deterministic_batch(&id))
                .map_err(|e| ch_errors::HorizonError::Invalid(e.to_string()))?
        ),
        Command::Ablate => {
            let b = ch_evaluation::ablations(&[0.99, 0.96, 0.88, 0.81], 0.95)?;
            println!(
                "combined={:.4} without_calibration={:.4} without_eprocess={:.4} ruvnet_only={:.4} enterprise_only={:.4}",
                b.combined,
                b.without_calibration,
                b.without_eprocess,
                b.ruvnet_only,
                b.enterprise_only
            )
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1)
    }
}
