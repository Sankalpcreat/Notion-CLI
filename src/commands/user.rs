use crate::client;
use crate::credentials;
use anyhow::Result;
use clap::Command;

pub fn command() -> Command {
    Command::new("user")
        .about("User operations")
        .subcommand(Command::new("me").about("Get bot user"))
        .subcommand(Command::new("get").about("Get user by ID").arg(clap::arg!(<user_id>)))
        .subcommand(
            Command::new("list")
                .about("List users")
                .arg(clap::arg!(--pagesize <N>).default_value("100"))
                .arg(clap::arg!(--startcursor <C> "Cursor for next page"))
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let token = if token.is_empty() { credentials::load()? } else { token.to_string() };
    match m.subcommand() {
        Some(("me", _)) => {
            let out = client::get(&token, "/users/me", None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("get", sub)) => {
            let id = sub.get_one::<String>("user_id").unwrap();
            let out = client::get(&token, &format!("/users/{}", id), None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("list", sub)) => {
            let ps = sub.get_one::<String>("pagesize").map(|s| s.as_str()).unwrap_or("100");
            let mut query: Vec<(&str, &str)> = vec![("page_size", ps)];
            if let Some(c) = sub.get_one::<String>("startcursor") {
                query.push(("start_cursor", c.as_str()));
            }
            let out = client::get(&token, "/users", Some(&query))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
