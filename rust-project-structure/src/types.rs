//! Ground truth: <https://github.com/rust-lang/team/blob/master/docs/toml-schema.md>.
//!
//! Last compared against `rust-lang/team` commit `547a4a2`.
//!
//! # Notion of a "team"
//!
//! Note that the "team" here refers purely to the implementation sense in `rust-lang/team`, not a
//! "team" in the actual Rust organization / decision-making sense. Example distinction:
//!
//! - `wg-embedded` is a team in `rust-lang/team` implementation wise.
//! - `wg-embedded` is not a team in project decision-making wise (AFAIK).
//!
//! In this module, when we say "team", we mean the concept implementation wise.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// A `rust-lang/team` team.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Team {
    /// Name of the team, used for GitHub (required).
    pub name: String,

    /// Name of the parent team of this team (optional).
    pub subteam_of: Option<String>,

    /// Is this team a top-level team, with a representative on the leadership-council?
    #[serde(default)]
    pub top_level: bool,

    /// Kind of team.
    #[serde(default)]
    pub kind: TeamKind,

    /// Constituent members (and alumni) of this team.
    pub people: People,
}

/// Kind of team.
///
/// # Remark on Working Groups and Project Groups
///
/// Note that [Working Groups (WGs)][TeamKind::WorkingGroup] and [Project Groups
/// (PGs)][TeamKind::ProjectGroup] are AFAIK in the process of being phased out: made into proper
/// sub-teams, retired, archived, etc.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TeamKind {
    /// Usually, [`TeamKind::Team`] are teams who maintain something, or participate in some kind of
    /// decision-making, or otherwise like `launching-pad` is an umbrella team for Leadership
    /// Council representation purposes.
    #[default]
    Team,
    /// [`TeamKind::WorkingGroup`] are teams which have an interest in some specific aspects or have
    /// some specific focus area. Typically, working groups don't participate in decision-making,
    /// but there are exceptions:
    ///
    /// - `wg-const-eval` is in practice more like a [`TeamKind::Team`], because `wg-const-eval`
    ///   vibe-checks and signs off of const stabilizations.
    WorkingGroup,
    /// [`TeamKind::WorkingGroup`] are teams which focus on achieving some kind of deliverable or
    /// goal. Those are usually part of a [`TeamKind::Team`] anyway.
    ProjectGroup,
    /// These are auxiliary teams that don't participate in decision-making or maintenance, usually
    /// only used for e.g. dev-desktop access or ping groups.
    MarkerTeam,
}

/// `[people]` section.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct People {
    /// Leads of the team, can be more than one and must be members of the team.
    ///
    /// Required, but it can be empty.
    pub leads: BTreeSet<Person>,
    /// Members of the team, can be empty.
    pub members: BTreeSet<Person>,

    /// (Optional) name of other teams whose members will be included as members of this team.
    /// Defaults to empty.
    #[serde(default)]
    pub included_teams: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Person {
    Simple(String),
    WithMeta {
        github: String,
        /// Can be empty. For instance, compiler maintainers will have `roles = ["maintainers"]`.
        roles: BTreeSet<String>,
    },
}

impl Person {
    pub fn github(&self) -> &str {
        match self {
            Person::Simple(github) => github,
            Person::WithMeta { github, .. } => github,
        }
    }

    pub fn roles(&self) -> Option<&BTreeSet<String>> {
        match self {
            Person::Simple(..) => None,
            Person::WithMeta { roles, .. } => Some(roles),
        }
    }
}
