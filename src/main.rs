mod client;
mod commands;
mod models;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
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
        #[command(subcommand)]
        subcommand: Option<OrgsSubcommand>,

        /// Number of orgs to fetch
        #[arg(short, long, default_value = "100")]
        limit: u32,

        /// Fuzzy search by name
        #[arg(short, long)]
        fuzzy: Option<String>,

        /// Output only org IDs (one per line)
        #[arg(short, long)]
        ids_only: bool,
    },

    /// Generate a sign-in link to impersonate a user
    Impersonate {
        /// User ID to impersonate (interactive if omitted)
        user_id: Option<String>,
    },

    /// Generate a JWT for API testing
    Jwt {
        /// User ID (interactive if omitted)
        user_id: Option<String>,

        /// JWT template name (interactive if omitted)
        #[arg(short, long)]
        template: Option<String>,

        /// List available templates
        #[arg(long)]
        list: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
enum OrgsSubcommand {
    /// Interactively pick an organization and print its ID
    Pick,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Users { limit, query } => {
            commands::users::run(limit, query).await?;
        }
        Commands::Orgs {
            subcommand,
            limit,
            fuzzy,
            ids_only,
        } => {
            if let Some(OrgsSubcommand::Pick) = subcommand {
                commands::orgs::pick().await?;
            } else {
                commands::orgs::run(limit, fuzzy, ids_only).await?;
            }
        }
        Commands::Impersonate { user_id } => {
            commands::impersonate::run(user_id).await?;
        }
        Commands::Jwt {
            user_id,
            template,
            list,
        } => {
            if list {
                commands::jwt::run_list_templates().await?;
            } else {
                commands::jwt::run(user_id, template).await?;
            }
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "clerk", &mut io::stdout());
        }
    }

    Ok(())
}
