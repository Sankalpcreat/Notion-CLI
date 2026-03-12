use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("token")
        .about("Token operations")
        .subcommand(Command::new("introspect").about("Introspect token"))
        .subcommand(Command::new("revoke").about("Revoke token"))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("introspect", _)) => {
            let body = json!({ "token": token });
            let client = reqwest::blocking::Client::new();
            let resp = client
                .post("https://api.notion.com/v1/tokens/introspect")
                .header("Notion-Version", "2026-03-11")
                .json(&body)
                .send()?;
            let out: serde_json::Value = resp.json()?;
            if out.get("object").and_then(|v| v.as_str()) == Some("error") {
                anyhow::bail!("{}", serde_json::to_string_pretty(&out)?);
            }
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
        Some(("revoke", _)) => {
            let body = json!({ "token": token });
            let client = reqwest::blocking::Client::new();
            let resp = client
                .post("https://api.notion.com/v1/tokens/revoke")
                .header("Notion-Version", "2026-03-11")
                .json(&body)
                .send()?;
            let out: serde_json::Value = resp.json()?;
            if out.get("object").and_then(|v| v.as_str()) == Some("error") {
                anyhow::bail!("{}", serde_json::to_string_pretty(&out)?);
            }
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
        _ => {}
    }
    Ok(())
}
