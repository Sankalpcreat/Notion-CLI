use anyhow::Result;
use clap::Command;
use reqwest::blocking::Client;
use serde_json::json;

pub fn command() -> Command {
    Command::new("oauth")
        .about("OAuth operations")
        .subcommand(
            Command::new("token")
                .about("Exchange code for access token")
                .arg(clap::arg!(--code <C> "Authorization code"))
                .arg(clap::arg!(--redirecturi <U> "Redirect URI"))
                .arg(clap::arg!(--clientid <ID> "Client ID"))
                .arg(clap::arg!(--clientsecret <S> "Client secret"))
        )
        .subcommand(
            Command::new("refresh")
                .about("Refresh access token")
                .arg(clap::arg!(--refreshtoken <T> "Refresh token"))
                .arg(clap::arg!(--clientid <ID> "Client ID"))
                .arg(clap::arg!(--clientsecret <S> "Client secret"))
        )
}

pub fn run(m: &clap::ArgMatches) -> Result<()> {
    match m.subcommand() {
        Some(("token", sub)) => {
            let client_id = std::env::var("NOTION_CLIENT_ID")
                .or_else(|_| sub.get_one::<String>("clientid").cloned().ok_or_else(|| anyhow::anyhow!("NOTION_CLIENT_ID or --clientid required")))?;
            let client_secret = std::env::var("NOTION_CLIENT_SECRET")
                .or_else(|_| sub.get_one::<String>("clientsecret").cloned().ok_or_else(|| anyhow::anyhow!("NOTION_CLIENT_SECRET or --clientsecret required")))?;
            let code = sub.get_one::<String>("code").ok_or_else(|| anyhow::anyhow!("--code required"))?;
            let redirect_uri = sub.get_one::<String>("redirecturi").ok_or_else(|| anyhow::anyhow!("--redirecturi required"))?;
            let body = json!({
                "grant_type": "authorization_code",
                "code": code,
                "redirect_uri": redirect_uri
            });
            let client = Client::new();
            let resp = client
                .post("https://api.notion.com/v1/oauth/token")
                .header("Notion-Version", "2026-03-11")
                .basic_auth(&client_id, Some(&client_secret))
                .json(&body)
                .send()?;
            let out: serde_json::Value = resp.json()?;
            if out.get("error").is_some() {
                anyhow::bail!("{}", serde_json::to_string_pretty(&out)?);
            }
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
        Some(("refresh", sub)) => {
            let client_id = std::env::var("NOTION_CLIENT_ID")
                .or_else(|_| sub.get_one::<String>("clientid").cloned().ok_or_else(|| anyhow::anyhow!("NOTION_CLIENT_ID or --clientid required")))?;
            let client_secret = std::env::var("NOTION_CLIENT_SECRET")
                .or_else(|_| sub.get_one::<String>("clientsecret").cloned().ok_or_else(|| anyhow::anyhow!("NOTION_CLIENT_SECRET or --clientsecret required")))?;
            let refresh_token = sub.get_one::<String>("refreshtoken").ok_or_else(|| anyhow::anyhow!("--refreshtoken required"))?;
            let body = json!({
                "grant_type": "refresh_token",
                "refresh_token": refresh_token
            });
            let client = Client::new();
            let resp = client
                .post("https://api.notion.com/v1/oauth/token")
                .header("Notion-Version", "2026-03-11")
                .basic_auth(&client_id, Some(&client_secret))
                .json(&body)
                .send()?;
            let out: serde_json::Value = resp.json()?;
            if out.get("error").is_some() {
                anyhow::bail!("{}", serde_json::to_string_pretty(&out)?);
            }
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
        _ => {}
    }
    Ok(())
}
