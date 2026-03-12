use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("comment")
        .about("Comment operations")
        .subcommand(
            Command::new("create")
                .about("Create a comment")
                .arg(clap::arg!(--pageid <ID> "Page ID"))
                .arg(clap::arg!(--blockid <ID> "Block ID (alternative to pageid)"))
                .arg(clap::arg!(--discussionid <ID> "Reply to discussion"))
                .arg(clap::arg!(--text <T> "Comment text"))
        )
        .subcommand(
            Command::new("list")
                .about("List comments")
                .arg(clap::arg!(--blockid <ID> "Block ID"))
                .arg(clap::arg!(--pageid <ID> "Page ID (alternative to blockid)"))
                .arg(clap::arg!(--pagesize <N>).default_value("100"))
                .arg(clap::arg!(--startcursor <C> "Cursor for next page"))
        )
        .subcommand(Command::new("get").about("Get a comment").arg(clap::arg!(<comment_id>)))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("create", sub)) => {
            let text = sub.get_one::<String>("text").ok_or_else(|| anyhow::anyhow!("--text required"))?;
            let mut body = json!({
                "rich_text": [{ "type": "text", "text": { "content": text } }]
            });
            if let Some(pid) = sub.get_one::<String>("pageid") {
                body["parent"] = json!({ "type": "page_id", "page_id": pid });
            } else if let Some(bid) = sub.get_one::<String>("blockid") {
                body["parent"] = json!({ "type": "block_id", "block_id": bid });
            } else if let Some(did) = sub.get_one::<String>("discussionid") {
                body["discussion_id"] = json!(did);
            } else {
                anyhow::bail!("--pageid, --blockid, or --discussionid required");
            }
            let out = client::post(&token, "/comments", Some(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("list", sub)) => {
            let mut query: Vec<(&str, &str)> = Vec::new();
            let bid = sub.get_one::<String>("blockid")
                .or_else(|| sub.get_one::<String>("pageid"))
                .ok_or_else(|| anyhow::anyhow!("--blockid or --pageid required"))?;
            query.push(("block_id", bid.as_str()));
            let ps = sub.get_one::<String>("pagesize").map(|s| s.as_str()).unwrap_or("100");
            query.push(("page_size", ps));
            if let Some(c) = sub.get_one::<String>("startcursor") {
                query.push(("start_cursor", c.as_str()));
            }
            let out = client::get(&token, "/comments", Some(&query))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("get", sub)) => {
            let id = sub.get_one::<String>("comment_id").unwrap();
            let out = client::get(&token, &format!("/comments/{}", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
