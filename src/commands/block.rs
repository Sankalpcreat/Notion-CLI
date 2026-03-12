use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("block")
        .about("Block operations")
        .subcommand(Command::new("get").about("Retrieve a block").arg(clap::arg!(<block_id>)))
        .subcommand(
            Command::new("children")
                .about("List block children")
                .arg(clap::arg!(<block_id>))
                .arg(clap::arg!(--pagesize <N>).default_value("100"))
                .arg(clap::arg!(--startcursor <C> "Cursor for next page"))
        )
        .subcommand(
            Command::new("append")
                .about("Append blocks")
                .arg(clap::arg!(<block_id>))
                .arg(clap::arg!(--text <T> "Paragraph text"))
                .arg(clap::arg!(--position <P> "start|end|after:<block_id>").default_value("end"))
        )
        .subcommand(
            Command::new("update")
                .about("Update a block")
                .arg(clap::arg!(<block_id>))
                .arg(clap::arg!(--text <T> "New text"))
        )
        .subcommand(Command::new("delete").about("Delete a block").arg(clap::arg!(<block_id>)))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("get", sub)) => {
            let id = sub.get_one::<String>("block_id").unwrap();
            let out = client::get(&token, &format!("/blocks/{}", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("children", sub)) => {
            let id = sub.get_one::<String>("block_id").unwrap();
            let ps = sub.get_one::<String>("pagesize").map(|s| s.as_str()).unwrap_or("100");
            let mut query: Vec<(&str, &str)> = vec![("page_size", ps)];
            if let Some(c) = sub.get_one::<String>("startcursor") {
                query.push(("start_cursor", c.as_str()));
            }
            let out = client::get(&token, &format!("/blocks/{}/children", id), Some(&query))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("append", sub)) => {
            let id = sub.get_one::<String>("block_id").unwrap();
            let text = sub.get_one::<String>("text").map(|s| s.as_str()).unwrap_or("");
            let pos = sub.get_one::<String>("position").map(|s| s.as_str()).unwrap_or("end");
            let mut body = json!({
                "children": [{
                    "object": "block",
                    "type": "paragraph",
                    "paragraph": {
                        "rich_text": [{ "type": "text", "text": { "content": text } }]
                    }
                }]
            });
            if pos.starts_with("after:") {
                let after_id = pos.strip_prefix("after:").unwrap_or("");
                body.as_object_mut().unwrap().insert("position".to_string(), json!({ "type": "after_block", "after_block": { "id": after_id } }));
            } else if pos == "start" {
                body.as_object_mut().unwrap().insert("position".to_string(), json!({ "type": "start" }));
            }
            let out = client::patch(&token, &format!("/blocks/{}/children", id), body)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("update", sub)) => {
            let id = sub.get_one::<String>("block_id").unwrap();
            let text = sub.get_one::<String>("text").ok_or_else(|| anyhow::anyhow!("--text required"))?;
            let body = json!({
                "paragraph": {
                    "rich_text": [{ "type": "text", "text": { "content": text } }]
                }
            });
            let out = client::patch(&token, &format!("/blocks/{}", id), body)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("delete", sub)) => {
            let id = sub.get_one::<String>("block_id").unwrap();
            let out = client::delete(&token, &format!("/blocks/{}", id))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
