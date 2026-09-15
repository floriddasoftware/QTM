use clap::{Parser, Subcommand};
use qp_hd::qp44::TotalMass;
use qp_hd::commands;

#[derive(Parser)]
#[command(name = "qp")]
#[command(version = "0.1.0")]
#[command(about = "QuantPerm Observer Runtime")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Qtm {
        #[command(subcommand)]
        command: QtmCommands,
    },
}

#[derive(Subcommand)]
enum QtmCommands {
    Create {
        name: String,

        #[arg(long)]
        indices: Option<String>,

        #[arg(long)]
        entropy: Option<String>,
    },

    Open {
        name: String,
    },

    Transit {
        name: String,

        #[arg(long, default_value_t = 0)]
        activation: u64,

        #[arg(long, default_value_t = 0)]
        purpose: u128,

        #[arg(long, default_value_t = 0)]
        coin: u128,

        #[arg(long, default_value_t = 0)]
        account: u128,

        #[arg(long, default_value_t = 0)]
        change: u128,

        #[arg(long, default_value_t = 0)]
        external: u128,
    },

    Exile {
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Qtm { command } => match command {
    
            QtmCommands::Create {
                name,
                indices,
                entropy,
            } => {
                commands::qtm_create(
                    name,
                    indices,
                    entropy,
                );
            }
    
            QtmCommands::Open {
                name,
            } => {
                commands::qtm_open(name);
            }
    
            QtmCommands::Transit {
                activation,
                name,
                purpose,
                coin,
                account,
                change,
                external,
            } => {
    
                let mass = TotalMass::new(
                    purpose,
                    coin,
                    account,
                    change,
                    external,
                );
                
                let payload: u128 = mass.memorize();
                
                commands::qtm_transit(
                    &name,
                    payload,
                    activation,
                );
            }
    
            QtmCommands::Exile { name } => {
                // Natively acts with just the namespace reference primitive
                commands::qtm_exile(&name);
            }
        },
    }
}