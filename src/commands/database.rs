use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("database")
        .about("Database operations")
        .subcommand(
            Command::new("create")
                .about("Create a database")
                .arg(clap::arg!(--parent <ID> "Parent page_id"))
                .arg(clap::arg!(--title <T> "Database title"))
        )
        .subcommand(Command::new("get").about("Retrieve a database").arg(clap::arg!(<database_id>)))
        .subcommand(
            Command::new("update")
                .about("Update a database")
                .arg(clap::arg!(<database_id>))
                .arg(clap::arg!(--title <T> "New title"))
                .arg(clap::arg!(--description <D> "Description"))
                .arg(clap::arg!(--icon <I> "Emoji or external URL"))
                .arg(clap::arg!(--cover <C> "Cover image URL"))
        )
        .subcommand(
            Command::new("query")
                .about("Query a database (deprecated; use datasource query)")
                .arg(clap::arg!(<database_id>))
                .arg(clap::arg!(--pagesize <N>).default_value("100"))
                .arg(clap::arg!(--startcursor <C> "Cursor for next page"))
                .arg(clap::arg!(--filter <JSON> "Filter JSON"))
                .arg(clap::arg!(--sorts <JSON> "Sorts JSON array"))
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("create", sub)) => {
            let parent = sub.get_one::<String>("parent").ok_or_else(|| anyhow::anyhow!("--parent required"))?;
            let title = sub.get_one::<String>("title").map(|s| s.as_str()).unwrap_or("Untitled");
            let body = json!({
                "parent": { "type": "page_id", "page_id": parent },
                "title": [{ "type": "text", "text": { "content": title } }],
                "properties": {
                    "Name": { "title": {} }
                }
            });
            let out = client::post(&token, "/databases", Some(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("get", sub)) => {
            let id = sub.get_one::<String>("database_id").unwrap();
            let out = client::get(&token, &format!("/databases/{}", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("update", sub)) => {
            let id = sub.get_one::<String>("database_id").unwrap();
            let mut body = serde_json::Map::new();
            if let Some(title) = sub.get_one::<String>("title") {
                body.insert("title".to_string(), json!([{ "type": "text", "text": { "content": title } }]));
            }
            if let Some(desc) = sub.get_one::<String>("description") {
                body.insert("description".to_string(), json!([{ "type": "text", "text": { "content": desc } }]));
            }
            if let Some(icon) = sub.get_one::<String>("icon") {
                let icon_val = if icon.starts_with("http://") || icon.starts_with("https://") {
                    json!({ "type": "external", "external": { "url": icon } })
                } else {
                    json!({ "type": "emoji", "emoji": icon })
                };
                body.insert("icon".to_string(), icon_val);
            }
            if let Some(cover) = sub.get_one::<String>("cover") {
                body.insert("cover".to_string(), json!({ "type": "external", "external": { "url": cover } }));
            }
            if body.is_empty() {
                anyhow::bail!("at least one of --title, --description, --icon, --cover required");
            }
            let out = client::patch(&token, &format!("/databases/{}", id), json!(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("query", sub)) => {
            let id = sub.get_one::<String>("database_id").unwrap();
            let ps = sub.get_one::<String>("pagesize").map(|s| s.as_str()).unwrap_or("100");
            let mut body = serde_json::Map::new();
            body.insert("page_size".to_string(), json!(ps.parse::<u32>().unwrap_or(100)));
            if let Some(c) = sub.get_one::<String>("startcursor") {
                body.insert("start_cursor".to_string(), json!(c));
            }
            if let Some(f) = sub.get_one::<String>("filter") {
                let filter: serde_json::Value = serde_json::from_str(f).map_err(|e| anyhow::anyhow!("invalid --filter JSON: {}", e))?;
                body.insert("filter".to_string(), filter);
            }
            if let Some(s) = sub.get_one::<String>("sorts") {
                let sorts: serde_json::Value = serde_json::from_str(s).map_err(|e| anyhow::anyhow!("invalid --sorts JSON: {}", e))?;
                body.insert("sorts".to_string(), sorts);
            }
            let out = client::post(&token, &format!("/databases/{}/query", id), Some(json!(body)))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
