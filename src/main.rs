mod client;
mod commands;
mod models;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::io;

#[derive(Parser)]
#[command(name = "clerk")]
#[command(about = "Unofficial Clerk CLI for admin tasks", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List and search users
    Users {
        /// Number of users to fetch
        #[arg(short, long, default_value = "10")]
        limit: u32,

        /// Search query (email/name)
        #[arg(short, long)]
        query: Option<String>,
    },

    /// List and fuzzy search organizations
    Orgs {
        /// Number of orgs to fetch
        #[arg(short, long, default_value = "100")]
        limit: u32,

        /// Fuzzy search by name
        #[arg(short, long)]
        fuzzy: Option<String>,
    },

    /// Generate a sign-in link to impersonate a user
    Impersonate {
        /// User ID to impersonate (interactive if omitted)
        user_id: Option<String>,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Users { limit, query } => {
            commands::users::run(limit, query).await?;
        }
        Commands::Orgs { limit, fuzzy } => {
            commands::orgs::run(limit, fuzzy).await?;
        }
        Commands::Impersonate { user_id } => {
            commands::impersonate::run(user_id).await?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "clerk", &mut io::stdout());
        }
    }

    Ok(())
}
