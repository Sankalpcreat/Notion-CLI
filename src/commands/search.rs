use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("search")
        .about("Search pages and data sources")
        .arg(clap::arg!(-q --query <Q> "Search query"))
        .arg(clap::arg!(--filter <TYPE> "Filter: page or data_source"))
        .arg(clap::arg!(--pagesize <N>).default_value("100"))
        .arg(clap::arg!(--startcursor <C> "Cursor for next page"))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    let mut body = serde_json::Map::new();
    if let Some(q) = m.get_one::<String>("query") {
        body.insert("query".to_string(), json!(q));
    }
    if let Some(f) = m.get_one::<String>("filter") {
        body.insert("filter".to_string(), json!({
            "property": "object",
            "value": f
        }));
    }
    if let Some(ps) = m.get_one::<String>("pagesize") {
        body.insert("page_size".to_string(), json!(ps.parse::<u32>().unwrap_or(100)));
    }
    if let Some(c) = m.get_one::<String>("startcursor") {
        body.insert("start_cursor".to_string(), json!(c));
    }
    let out = client::post(&token, "/search", Some(json!(body)))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
