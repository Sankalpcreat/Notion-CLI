mod block;
mod comment;
mod database;
mod datasource;
mod file;
mod oauth;
mod page;
mod search;
mod token;
mod user;

use crate::credentials;
use clap::Command;

pub fn build_cli() -> Command {
    Command::new("notion-cli")
        .about("Notion API CLI")
        .subcommand(page::command())
        .subcommand(block::command())
        .subcommand(database::command())
        .subcommand(datasource::command())
        .subcommand(comment::command())
        .subcommand(user::command())
        .subcommand(search::command())
        .subcommand(file::command())
        .subcommand(oauth::command())
        .subcommand(token::command())
}

pub fn run(matches: &clap::ArgMatches) -> anyhow::Result<()> {
    let token = credentials::load().unwrap_or_default();
    match matches.subcommand() {
        Some(("page", m)) => page::run(m, &token),
        Some(("block", m)) => block::run(m, &token),
        Some(("database", m)) => database::run(m, &token),
        Some(("datasource", m)) => datasource::run(m, &token),
        Some(("comment", m)) => comment::run(m, &token),
        Some(("user", m)) => user::run(m, &token),
        Some(("search", m)) => search::run(m, &token),
        Some(("file", m)) => file::run(m, &token),
        Some(("oauth", m)) => oauth::run(m),
        Some(("token", m)) => token::run(m, &token),
        _ => Ok(()),
    }
}
