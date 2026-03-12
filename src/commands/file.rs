use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;
use serde_json::json;

pub fn command() -> Command {
    Command::new("file")
        .about("File upload operations")
        .subcommand(
            Command::new("create")
                .about("Create file upload")
                .arg(clap::arg!(--path <P> "File path"))
                .arg(clap::arg!(--filename <N> "File name override"))
                .arg(clap::arg!(--"content-type" <M> "MIME type").default_value("application/octet-stream"))
                .arg(clap::arg!(--mode <MODE> "single_part or multi_part").default_value("single_part"))
                .arg(clap::arg!(--parts <N> "Number of parts for multi_part mode"))
        )
        .subcommand(
            Command::new("list")
                .about("List file uploads")
        )
        .subcommand(Command::new("get").about("Get file upload status").arg(clap::arg!(<file_upload_id>)))
        .subcommand(
            Command::new("send")
                .about("Send file part (multi-part upload)")
                .arg(clap::arg!(<file_upload_id>))
                .arg(clap::arg!(--path <P> "File path"))
                .arg(clap::arg!(--part <N> "Part number (1-1000, for multi_part mode)"))
        )
        .subcommand(
            Command::new("complete")
                .about("Complete multi-part file upload")
                .arg(clap::arg!(<file_upload_id>))
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("create", sub)) => {
            let path = sub.get_one::<String>("path").ok_or_else(|| anyhow::anyhow!("--path required"))?;
            let meta = std::fs::metadata(path)?;
            let filename = sub
                .get_one::<String>("filename")
                .cloned()
                .unwrap_or_else(|| {
                    std::path::Path::new(path)
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or("file")
                        .to_string()
                });
            let mode = sub
                .get_one::<String>("mode")
                .map(|s| s.as_str())
                .unwrap_or("single_part");
            let mut body = serde_json::Map::new();
            body.insert("filename".to_string(), json!(filename));
            body.insert(
                "content_type".to_string(),
                json!(
                    sub.get_one::<String>("content-type")
                        .map(|s| s.as_str())
                        .unwrap_or("application/octet-stream")
                ),
            );
            body.insert("size".to_string(), json!(meta.len()));
            body.insert("mode".to_string(), json!(mode));
            if mode == "multi_part" {
                let parts = sub
                    .get_one::<String>("parts")
                    .ok_or_else(|| anyhow::anyhow!("--parts required when --mode multi_part"))?
                    .parse::<u32>()
                    .map_err(|_| anyhow::anyhow!("--parts must be a positive integer"))?;
                body.insert("number_of_parts".to_string(), json!(parts));
            }
            let out = client::post(&token, "/file_uploads", Some(json!(body)))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("list", _)) => {
            let out = client::get(&token, "/file_uploads", None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("get", sub)) => {
            let id = sub.get_one::<String>("file_upload_id").unwrap();
            let out = client::get(&token, &format!("/file_uploads/{}", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("send", sub)) => {
            let id = sub.get_one::<String>("file_upload_id").unwrap();
            let path = sub.get_one::<String>("path").ok_or_else(|| anyhow::anyhow!("--path required"))?;
            let part = sub.get_one::<String>("part").and_then(|s| s.parse::<u32>().ok());
            let out = client::post_multipart(&token, &format!("/file_uploads/{}/send", id), path, part)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("complete", sub)) => {
            let id = sub.get_one::<String>("file_upload_id").unwrap();
            let out = client::post(&token, &format!("/file_uploads/{}/complete", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
