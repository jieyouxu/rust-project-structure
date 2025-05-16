use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    /// Path to root of an up-to-date `rust-lang/team` checkout.
    #[clap(short = 't', long)]
    pub team_repo_path: Utf8PathBuf,

    /// Kind of graph to generate.
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Include Working Groups and Project Groups, but exclude Marker Teams.
    ExcludeMarkerTeams,
    /// Include *only* formal teams.
    OnlyTeams,
}
