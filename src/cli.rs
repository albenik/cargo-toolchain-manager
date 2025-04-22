use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(arg_required_else_help = true)]
#[command(subcommand_required = true)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub enum CargoCli {
    ToolchainManager(ToolchainManagerArgs),
}

#[derive(Args)]
#[command(author, version, about)]
pub struct ToolchainManagerArgs {
    #[command(subcommand)]
    pub commands: ToolchainManagerCommands,
}

#[derive(Subcommand)]
pub enum ToolchainManagerCommands {
    /// Install multiple toolchain versions
    Install(InstallArgs),
}

#[derive(Args)]
pub struct InstallArgs {
    /// Only display output without installing
    #[arg(long)]
    pub dry_run: bool,

    /// Semver version range (e.g., '>=1.60,<1.70')
    #[arg(long)]
    pub range: Option<String>,
}
