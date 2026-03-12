use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;

const BASE_URL: &str = "https://api.notion.com/v1";
const NOTION_VERSION: &str = "2026-03-11";

pub fn get(token: &str, path: &str, query: Option<&[(&str, &str)]>) -> Result<Vec<u8>> {
    let url = format!("{}{}", BASE_URL, path);
    let client = Client::new();
    let mut req = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", NOTION_VERSION);
    if let Some(q) = query {
        req = req.query(q);
    }
    let resp = req.send().context("request failed")?;
    let body = resp.bytes().context("read body")?;
    check_error(&body)?;
    Ok(serde_json::to_vec_pretty(&serde_json::from_slice::<Value>(&body)?)?)
}

pub fn post(token: &str, path: &str, body: Option<Value>) -> Result<Vec<u8>> {
    let url = format!("{}{}", BASE_URL, path);
    let client = Client::new();
    let mut req = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", NOTION_VERSION)
        .header("Content-Type", "application/json");
    if let Some(b) = body {
        req = req.json(&b);
    }
    let resp = req.send().context("request failed")?;
    let body = resp.bytes().context("read body")?;
    check_error(&body)?;
    Ok(serde_json::to_vec_pretty(&serde_json::from_slice::<Value>(&body)?)?)
}

pub fn patch(token: &str, path: &str, body: Value) -> Result<Vec<u8>> {
    let url = format!("{}{}", BASE_URL, path);
    let client = Client::new();
    let resp = client
        .patch(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", NOTION_VERSION)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .context("request failed")?;
    let body = resp.bytes().context("read body")?;
    check_error(&body)?;
    Ok(serde_json::to_vec_pretty(&serde_json::from_slice::<Value>(&body)?)?)
}

pub fn post_multipart(token: &str, path: &str, file_path: &str, part_number: Option<u32>) -> Result<Vec<u8>> {
    let url = format!("{}{}", BASE_URL, path);
    let client = Client::new();
    let mut form = reqwest::blocking::multipart::Form::new()
        .file("file", file_path)
        .context("open file")?;
    if let Some(n) = part_number {
        form = form.text("part_number", n.to_string());
    }
    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", NOTION_VERSION)
        .multipart(form)
        .send()
        .context("request failed")?;
    let body = resp.bytes().context("read body")?;
    check_error(&body)?;
    Ok(serde_json::to_vec_pretty(&serde_json::from_slice::<Value>(&body)?)?)
}

pub fn delete(token: &str, path: &str) -> Result<Vec<u8>> {
    let url = format!("{}{}", BASE_URL, path);
    let client = Client::new();
    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", NOTION_VERSION)
        .send()
        .context("request failed")?;
    let body = resp.bytes().context("read body")?;
    check_error(&body)?;
    Ok(serde_json::to_vec_pretty(&serde_json::from_slice::<Value>(&body)?)?)
}

fn check_error(body: &[u8]) -> Result<()> {
    if let Ok(v) = serde_json::from_slice::<Value>(body) {
        if v.get("object").and_then(|o| o.as_str()) == Some("error") {
            let code = v.get("code").and_then(|c| c.as_str()).unwrap_or("unknown");
            let msg = v.get("message").and_then(|m| m.as_str()).unwrap_or("");
            anyhow::bail!("notion api error [{}]: {}", code, msg);
        }
    }
    Ok(())
}
