use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Path to root of an up-to-date `rust-lang/team` checkout.
    #[clap(short = 't', long)]
    pub team_repo_path: Utf8PathBuf,
}
