use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn load() -> Result<String> {
    if let Ok(t) = std::env::var("NOTION_API_KEY") {
        if !t.is_empty() {
            return Ok(t);
        }
    }
    if let Ok(t) = std::env::var("NOTION_TOKEN") {
        if !t.is_empty() {
            return Ok(t);
        }
    }
    let dir = dirs::home_dir().context("no home dir")?;
    let path: PathBuf = [dir, ".notion".into(), "credentials.json".into()]
        .iter()
        .collect();
    if path.exists() {
        let data: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).context("read credentials")?)
                .context("parse credentials")?;
        if let Some(t) = data.get("token").and_then(|v| v.as_str()) {
            if !t.is_empty() {
                return Ok(t.to_string());
            }
        }
    }
    anyhow::bail!(
        "no token found. set NOTION_API_KEY or create ~/.notion/credentials.json"
    )
}
