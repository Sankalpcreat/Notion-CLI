use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("page")
        .about("Page operations")
        .subcommand(
            Command::new("create")
                .about("Create a page")
                .arg(clap::arg!(--parent <ID> "Parent page_id or database_id"))
                .arg(clap::arg!(--title <T> "Page title"))
        )
        .subcommand(
            Command::new("get")
                .about("Retrieve a page")
                .arg(clap::arg!(<page_id>))
                .arg(
                    clap::arg!(--"filter-properties" <ID> "Filter returned properties by ID")
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Update a page")
                .arg(clap::arg!(<page_id>))
                .arg(clap::arg!(--title <T> "New title"))
                .arg(clap::arg!(--trash [BOOL] "Trash or restore"))
                .arg(clap::arg!(--icon <I> "Emoji or external URL"))
                .arg(clap::arg!(--cover <C> "Cover image URL"))
                .arg(clap::arg!(--lock [BOOL] "Lock page"))
                .arg(clap::arg!(--template [BOOL] "Set as template"))
                .arg(clap::arg!(--"erase-content" [BOOL] "Erase all block content"))
        )
        .subcommand(
            Command::new("move")
                .about("Move a page")
                .arg(clap::arg!(<page_id>))
                .arg(clap::arg!(--parent <ID> "New parent page_id or data_source_id"))
        )
        .subcommand(Command::new("markdown").about("Get page as markdown").arg(clap::arg!(<page_id>)))
        .subcommand(
            Command::new("markdown-update")
                .about("Insert/replace page content via markdown")
                .arg(clap::arg!(<page_id>))
                .arg(clap::arg!(--operation <OP> "replace|insert|replace-range|update").default_value("replace"))
                .arg(clap::arg!(--content <C> "Markdown content"))
                .arg(clap::arg!(--after <A> "Insert after (ellipsis: start...end)"))
                .arg(clap::arg!(--range <R> "Replace range (ellipsis: start...end)"))
                .arg(clap::arg!(--find <O> "String to find (for update)"))
                .arg(clap::arg!(--replace <N> "Replacement string (for update)"))
                .arg(clap::arg!(--permitdelete [BOOL] "Allow deleting child pages"))
        )
        .subcommand(
            Command::new("property")
                .about("Get page property by ID")
                .arg(clap::arg!(<page_id>))
                .arg(clap::arg!(<property_id>))
                .arg(clap::arg!(--pagesize <N>).default_value("100"))
                .arg(clap::arg!(--startcursor <C> "Cursor for next page"))
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("create", sub)) => {
            let parent = sub.get_one::<String>("parent").ok_or_else(|| anyhow::anyhow!("--parent required"))?;
            let title = sub.get_one::<String>("title").map(|s| s.as_str()).unwrap_or("Untitled");
            let body = json!({
                "parent": { "page_id": parent },
                "properties": {
                    "title": {
                        "title": [{ "text": { "content": title } }]
                    }
                }
            });
            let out = client::post(&token, "/pages", Some(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("get", sub)) => {
            let id = sub.get_one::<String>("page_id").unwrap();
            let query_pairs: Vec<(String, String)> = sub
                .get_many::<String>("filter-properties")
                .map(|vals| {
                    vals.map(|v| ("filter_properties".to_string(), v.clone()))
                        .collect()
                })
                .unwrap_or_default();
            let query_refs: Vec<(&str, &str)> = query_pairs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            let out = if query_refs.is_empty() {
                client::get(&token, &format!("/pages/{}", id), None)?
            } else {
                client::get(&token, &format!("/pages/{}", id), Some(&query_refs))?
            };
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("update", sub)) => {
            let id = sub.get_one::<String>("page_id").unwrap();
            let mut body = serde_json::Map::new();
            if let Some(t) = sub.get_one::<String>("title") {
                body.insert("properties".to_string(), json!({
                    "title": { "title": [{ "text": { "content": t } }] }
                }));
            }
            if let Some(trash) = sub.get_one::<String>("trash") {
                let b = trash == "true" || trash == "1" || trash.is_empty();
                body.insert("in_trash".to_string(), json!(b));
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
            if let Some(lock) = sub.get_one::<String>("lock") {
                let b = lock == "true" || lock == "1" || lock.is_empty();
                body.insert("is_locked".to_string(), json!(b));
            }
            if let Some(tmpl) = sub.get_one::<String>("template") {
                let b = tmpl == "true" || tmpl == "1" || tmpl.is_empty();
                body.insert("is_template".to_string(), json!(b));
            }
            if let Some(erase) = sub.get_one::<String>("erase-content") {
                let b = erase == "true" || erase == "1" || erase.is_empty();
                body.insert("erase_content".to_string(), json!(b));
            }
            let out = client::patch(&token, &format!("/pages/{}", id), json!(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("move", sub)) => {
            let id = sub.get_one::<String>("page_id").unwrap();
            let parent = sub.get_one::<String>("parent").ok_or_else(|| anyhow::anyhow!("--parent required"))?;
            let body = json!({
                "parent": { "type": "page_id", "page_id": parent }
            });
            let out = client::post(&token, &format!("/pages/{}/move", id), Some(body))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("markdown", sub)) => {
            let id = sub.get_one::<String>("page_id").unwrap();
            let out = client::get(&token, &format!("/pages/{}/markdown", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("markdown-update", sub)) => {
            let id = sub.get_one::<String>("page_id").unwrap();
            let content = sub.get_one::<String>("content").ok_or_else(|| anyhow::anyhow!("--content required"))?;
            let op = sub.get_one::<String>("operation").map(|s| s.as_str()).unwrap_or("replace");
            let allow = sub.get_one::<String>("permitdelete").map(|s| s == "true" || s == "1").unwrap_or(false);
            let body = match op {
                "insert" => {
                    let mut insert = serde_json::Map::new();
                    insert.insert("content".to_string(), json!(content));
                    if let Some(a) = sub.get_one::<String>("after") {
                        insert.insert("after".to_string(), json!(a));
                    }
                    json!({
                        "type": "insert_content",
                        "insert_content": insert
                    })
                }
                "replace-range" => {
                    let range = sub.get_one::<String>("range").ok_or_else(|| anyhow::anyhow!("--range required for replace-range"))?;
                    json!({
                        "type": "replace_content_range",
                        "replace_content_range": {
                            "content": content,
                            "content_range": range,
                            "allow_deleting_content": allow
                        }
                    })
                }
                "update" => {
                    let old = sub.get_one::<String>("find").ok_or_else(|| anyhow::anyhow!("--find required for update"))?;
                    let new_str = sub.get_one::<String>("replace").ok_or_else(|| anyhow::anyhow!("--replace required for update"))?;
                    json!({
                        "type": "update_content",
                        "update_content": {
                            "content_updates": [{ "old_str": old, "new_str": new_str }],
                            "allow_deleting_content": allow
                        }
                    })
                }
                _ => json!({
                    "type": "replace_content",
                    "replace_content": {
                        "new_str": content,
                        "allow_deleting_content": allow
                    }
                })
            };
            let out = client::patch(&token, &format!("/pages/{}/markdown", id), body)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("property", sub)) => {
            let page_id = sub.get_one::<String>("page_id").unwrap();
            let prop_id = sub.get_one::<String>("property_id").unwrap();
            let ps = sub.get_one::<String>("pagesize").map(|s| s.as_str()).unwrap_or("100");
            let mut query: Vec<(&str, &str)> = vec![("page_size", ps)];
            if let Some(c) = sub.get_one::<String>("startcursor") {
                query.push(("start_cursor", c.as_str()));
            }
            let out = client::get(&token, &format!("/pages/{}/properties/{}", page_id, prop_id), Some(&query))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
