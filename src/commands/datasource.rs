use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("datasource")
        .about("Data source operations")
        .subcommand(
            Command::new("create")
                .about("Create a data source")
                .arg(clap::arg!(--parent <ID> "Parent database_id"))
                .arg(clap::arg!(--title <T> "Data source title"))
        )
        .subcommand(Command::new("get").about("Retrieve a data source").arg(clap::arg!(<data_source_id>)))
        .subcommand(
            Command::new("update")
                .about("Update a data source")
                .arg(clap::arg!(<data_source_id>))
                .arg(clap::arg!(--title <T> "New title"))
                .arg(clap::arg!(--icon <I> "Emoji or external URL"))
                .arg(clap::arg!(--cover <C> "Cover image URL"))
                .arg(clap::arg!(--trash [BOOL] "Trash or restore data source"))
                .arg(clap::arg!(--parent <ID> "New parent ID"))
                .arg(
                    clap::arg!(--"parent-type" <TYPE> "Parent type: database_id or page_id")
                        .default_value("database_id"),
                )
                .arg(clap::arg!(--properties <JSON> "Properties schema JSON"))
        )
        .subcommand(
            Command::new("query")
                .about("Query a data source")
                .arg(clap::arg!(<data_source_id>))
                .arg(clap::arg!(--pagesize <N>).default_value("100"))
                .arg(clap::arg!(--startcursor <C> "Cursor for next page"))
                .arg(clap::arg!(--filter <JSON> "Filter JSON"))
                .arg(clap::arg!(--sorts <JSON> "Sorts JSON array"))
        )
        .subcommand(Command::new("templates").about("List data source templates").arg(clap::arg!(<data_source_id>)))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("create", sub)) => {
            let parent = sub.get_one::<String>("parent").ok_or_else(|| anyhow::anyhow!("--parent required"))?;
            let title = sub.get_one::<String>("title").map(|s| s.as_str()).unwrap_or("Untitled");
            let body = json!({
                "parent": { "type": "database_id", "database_id": parent },
                "title": [{ "type": "text", "text": { "content": title } }],
                "properties": {
                    "Name": { "title": {} }
                }
            });
            let out = client::post(&token, "/data_sources", Some(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("get", sub)) => {
            let id = sub.get_one::<String>("data_source_id").unwrap();
            let out = client::get(&token, &format!("/data_sources/{}", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("update", sub)) => {
            let id = sub.get_one::<String>("data_source_id").unwrap();
            let mut body = serde_json::Map::new();
            if let Some(title) = sub.get_one::<String>("title") {
                body.insert(
                    "title".to_string(),
                    json!([{ "type": "text", "text": { "content": title } }]),
                );
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
                body.insert(
                    "cover".to_string(),
                    json!({ "type": "external", "external": { "url": cover } }),
                );
            }
            if let Some(trash) = sub.get_one::<String>("trash") {
                let b = trash == "true" || trash == "1" || trash.is_empty();
                body.insert("in_trash".to_string(), json!(b));
            }
            if let Some(parent) = sub.get_one::<String>("parent") {
                let ptype = sub
                    .get_one::<String>("parent-type")
                    .map(|s| s.as_str())
                    .unwrap_or("database_id");
                body.insert(
                    "parent".to_string(),
                    json!({ "type": ptype, ptype: parent }),
                );
            }
            if let Some(props) = sub.get_one::<String>("properties") {
                let parsed: serde_json::Value = serde_json::from_str(props)
                    .map_err(|e| anyhow::anyhow!("invalid --properties JSON: {}", e))?;
                body.insert("properties".to_string(), parsed);
            }
            if body.is_empty() {
                anyhow::bail!(
                    "at least one update field is required (--title/--icon/--cover/--trash/--parent/--properties)"
                );
            }
            let out = client::patch(&token, &format!("/data_sources/{}", id), json!(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("query", sub)) => {
            let id = sub.get_one::<String>("data_source_id").unwrap();
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
            let out = client::post(&token, &format!("/data_sources/{}/query", id), Some(json!(body)))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("templates", sub)) => {
            let id = sub.get_one::<String>("data_source_id").unwrap();
            let out = client::get(&token, &format!("/data_sources/{}/templates", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
