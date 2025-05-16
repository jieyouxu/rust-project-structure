//! A small utility, that given a checkout of the `rust-lang/team` repository
//!
//! # Data assumptions
//!
//! - An up-to-date checkout of `rust-lang/team` repository (assume checkout root path is `$team`).
//! - Teams are `.toml` configuration files under `$team/teams/`, e.g. `apple.toml`.
//! - Each of such team `.toml` configuration file has top-level fields:
//!     - `name = "$team_name"`
//!     - When a subteam, `subteam-of = "$parent_team"`
//!     - A `[people]` section, under of which:
//!         - An array of team leads, `leads = []`.
//!         - An array of team members, `members = []`.
//!
//! # Current features
//!
//! Currently, this util is intended to be *very* limited in scope, and only aims to establish a DAG
//! of `subteam-of` relationship between teams.
//!
//! A future extension is to show team members (and distinction for team leads, and possible roles
//! for e.g. compiler maintainers?) for each team in such a DAG.

mod cli;
mod types;

use std::fs;

use anyhow::Context;
use camino::Utf8Path;
use clap::Parser;
use itertools::Itertools;
use log::{Level, debug, info, log_enabled};
use petgraph::Directed;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::GraphMap;
use types::TeamKind;
use walkdir::WalkDir;

use self::cli::Cli;
use self::types::Team;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .parse_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"))
        .init();

    let args = Cli::parse();
    debug!(args:?; "Parsed cli args");

    info!("Processing teams from given `rust-lang/team` checkout at `{}`", args.team_repo_path);

    let teams_path = args.team_repo_path.join("teams");
    debug!("Collecting team specs under `{teams_path}`");

    let mut teams = read_teams(&teams_path)
        .with_context(|| format!("failed to read team specs from `{teams_path}`"))?;
    debug_teams("all teams", &teams);

    // Drop marker teams...
    teams.retain(|t| t.kind != TeamKind::MarkerTeam);
    debug_teams("non-`marker-team`s", &teams);

    // Establish a DAG formed from "subteam-of" relationships.
    let mut subteam_of_graph: GraphMap<&str, &str, Directed> = GraphMap::new();

    // 1. Create the nodes. Nodes are non-marker teams (but incl. WGs and PGs).
    teams.iter().for_each(|team| {
        subteam_of_graph.add_node(&team.name);
    });

    // 2. Establish "subteam-of" relationships.
    for (team_a, team_b) in teams.iter().cartesian_product(teams.iter()) {
        if team_a
            .subteam_of
            .as_ref()
            .is_some_and(|parent_team_name| parent_team_name == &team_b.name)
        {
            subteam_of_graph.add_edge(&team_a.name, &team_b.name, "subteam of");
        }
    }

    println!(
        "{:?}",
        Dot::with_config(
            &subteam_of_graph,
            &[Config::EdgeNoLabel, Config::RankDir(petgraph::dot::RankDir::RL)]
        )
    );

    Ok(())
}

fn read_teams(teams_path: &Utf8Path) -> anyhow::Result<Vec<Team>> {
    let direct_descendants = WalkDir::new(teams_path)
        // Only yield direct descendants.
        .min_depth(1)
        .max_depth(1)
        .follow_root_links(false)
        .sort_by_file_name()
        .same_file_system(true);

    let team_specs_iter = direct_descendants.into_iter().filter_entry(|entry| {
        !entry.path_is_symlink()
            && entry.file_type().is_file()
            // Retain only `.toml` files.
            && entry.path().extension().is_some_and(|ext| ext == "toml")
    });

    let mut team_specs = vec![];

    for entry in team_specs_iter {
        let entry = entry.with_context(|| format!("failed to read file under `{teams_path}`"))?;
        let path =
            Utf8Path::from_path(entry.path()).expect("this tool does not handle non-UTF-8 paths");

        let team_spec_raw =
            fs::read_to_string(path).with_context(|| format!("failed to read `{path}`"))?;
        let team_spec: Team = toml::from_str(&team_spec_raw)
            .with_context(|| format!("failed to parse `{path}` as known `Team` TOML"))?;
        team_specs.push(team_spec);
    }

    Ok(team_specs)
}

fn debug_teams(msg: &'static str, teams: &[Team]) {
    if log_enabled!(Level::Debug) {
        for i in 0..5 {
            debug!(example_team:? = teams[i]; "{msg}: teams[{i}]");
        }
    }
}
