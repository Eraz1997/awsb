use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "awsb")]
#[command(about = "A CLI tool which boosts AWS SSO sessions and profiles management.")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Example command")]
    Example {
        #[arg(help = "Example argument")]
        example_argument: String,
        #[arg(short, long, help = "Example optional argument")]
        example_optional_argument: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Example {
            example_argument,
            example_optional_argument,
        } => {
            if let Some(example_optional_argument) = example_optional_argument {
                println!("Run with {example_argument} and optional {example_optional_argument}")
            } else {
                println!("Run with {example_argument}");
            }
        }
    }
}
