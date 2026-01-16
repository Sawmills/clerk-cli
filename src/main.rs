mod client;
mod commands;
mod models;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
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
    /// Manage users
    Users {
        #[command(subcommand)]
        subcommand: Option<UsersSubcommand>,

        /// User ID to act on
        user: Option<String>,
    },

    /// Manage organizations
    Orgs {
        #[command(subcommand)]
        subcommand: Option<OrgsSubcommand>,

        /// Organization slug or ID
        org: Option<String>,
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

    #[command(hide = true)]
    CompleteOrgs,

    #[command(hide = true)]
    CompleteUsers {
        #[arg(long)]
        org: Option<String>,
    },

    #[command(hide = true)]
    CompleteJwtTemplates,
}

#[derive(Subcommand)]
enum OrgsSubcommand {
    /// List all organizations
    List {
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
    /// Create a new organization
    Create {
        /// Organization name
        #[arg(short, long)]
        name: String,

        /// Organization slug (auto-generated if omitted)
        #[arg(short, long)]
        slug: Option<String>,
    },
    /// Interactively pick an organization and print its ID
    Pick,
    /// List members of the organization, or act on a specific member
    Members {
        /// User ID to act on (or 'add' to add a member)
        user_id: Option<String>,

        /// Action to perform on the user
        #[arg(value_enum)]
        action: Option<MemberAction>,

        /// JWT template name (for jwt action)
        template: Option<String>,

        /// User ID to add (for add action)
        #[arg(short = 'u', long = "user")]
        add_user_id: Option<String>,

        /// Role for the new member (for add action)
        #[arg(short, long, default_value = "org:member")]
        role: String,
    },
    /// Delete this organization
    Delete {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Clone, ValueEnum)]
enum MemberAction {
    Impersonate,
    Jwt,
    Add,
}

#[derive(Subcommand)]
enum UsersSubcommand {
    /// List and search users
    List {
        #[arg(short, long, default_value = "10")]
        limit: u32,

        #[arg(short, long)]
        query: Option<String>,
    },
    /// Create a new user
    Create {
        #[arg(short, long)]
        email: String,

        #[arg(short, long)]
        first_name: Option<String>,

        #[arg(short, long)]
        last_name: Option<String>,

        #[arg(short, long)]
        password: Option<String>,
    },
    /// Impersonate this user
    Impersonate,
    /// Generate a JWT for this user
    Jwt {
        template: Option<String>,
    },
    /// Add this user to an organization
    AddToOrg {
        #[arg(short, long)]
        org: String,

        #[arg(short, long, default_value = "org:member")]
        role: String,
    },
    /// Remove this user from an organization
    RemoveFromOrg {
        #[arg(short, long)]
        org: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Users { subcommand, user } => match (subcommand, user) {
            (Some(UsersSubcommand::List { limit, query }), _) => {
                commands::users::list(limit, query).await?;
            }
            (Some(UsersSubcommand::Create { email, first_name, last_name, password }), _) => {
                commands::users::create(email, first_name, last_name, password).await?;
            }
            (Some(UsersSubcommand::Impersonate), Some(user)) => {
                commands::impersonate::run(Some(user)).await?;
            }
            (Some(UsersSubcommand::Jwt { template }), Some(user)) => {
                commands::jwt::run(Some(user), template).await?;
            }
            (Some(UsersSubcommand::AddToOrg { org, role }), Some(user)) => {
                commands::users::add_to_org(&user, &org, &role).await?;
            }
            (Some(UsersSubcommand::RemoveFromOrg { org }), Some(user)) => {
                commands::users::remove_from_org(&user, &org).await?;
            }
            (Some(UsersSubcommand::Impersonate | UsersSubcommand::Jwt { .. } | UsersSubcommand::AddToOrg { .. } | UsersSubcommand::RemoveFromOrg { .. }), None) => {
                anyhow::bail!("User ID required. Usage: clerk users <user_id> <action>");
            }
            (None, Some(user)) => {
                commands::users::show(&user).await?;
            }
            (None, None) => {
                commands::users::list(10, None).await?;
            }
        },
        Commands::Orgs { subcommand, org } => match (subcommand, org) {
            (
                Some(OrgsSubcommand::List {
                    limit,
                    fuzzy,
                    ids_only,
                }),
                _,
            ) => {
                commands::orgs::run(limit, fuzzy, ids_only).await?;
            }
            (Some(OrgsSubcommand::Pick), _) => {
                commands::orgs::pick().await?;
            }
            (Some(OrgsSubcommand::Create { name, slug }), _) => {
                commands::orgs::create(name, slug).await?;
            }
            (
                Some(OrgsSubcommand::Members {
                    user_id,
                    action,
                    template,
                    add_user_id,
                    role,
                }),
                Some(org),
            ) => {
                commands::orgs::members(
                    &org,
                    user_id,
                    action.map(|a| match a {
                        MemberAction::Impersonate => commands::orgs::MemberAction::Impersonate,
                        MemberAction::Jwt => commands::orgs::MemberAction::Jwt(template),
                        MemberAction::Add => commands::orgs::MemberAction::Add {
                            user_id: add_user_id.expect("--user required for add action"),
                            role,
                        },
                    }),
                )
                .await?;
            }
            (Some(OrgsSubcommand::Members { .. }), None) => {
                anyhow::bail!(
                    "Organization slug required for 'members' command. Usage: clerk orgs <org> members"
                );
            }
            (Some(OrgsSubcommand::Delete { force }), Some(org)) => {
                commands::orgs::delete(&org, force).await?;
            }
            (Some(OrgsSubcommand::Delete { .. }), None) => {
                anyhow::bail!(
                    "Organization slug required for 'delete' command. Usage: clerk orgs <org> delete"
                );
            }
            (None, Some(org)) => {
                commands::orgs::show(&org).await?;
            }
            (None, None) => {
                commands::orgs::run(100, None, false).await?;
            }
        },
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
            if shell == Shell::Zsh {
                print!("{}", include_str!("completions/clerk.zsh"));
            } else {
                let mut cmd = Cli::command();
                generate(shell, &mut cmd, "clerk", &mut io::stdout());
            }
        }
        Commands::CompleteOrgs => {
            commands::completions::complete_orgs().await?;
        }
        Commands::CompleteUsers { org } => {
            commands::completions::complete_users(org).await?;
        }
        Commands::CompleteJwtTemplates => {
            commands::completions::complete_jwt_templates().await?;
        }
    }

    Ok(())
}
