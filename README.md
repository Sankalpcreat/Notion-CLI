# notion-cli

<p align="center"><img src="assets/notion-cli-banner.png" width="520" alt="Notion CLI" /></p>

> **Disclaimer:** This is not an official Notion project. Independently built using the public [Notion API](https://developers.notion.com/) and open-sourced for practical terminal and automation workflows. Not affiliated with or endorsed by Notion Labs, Inc.

Notion API CLI in Rust. 37 commands for pages, blocks, data sources, comments, users, search, file uploads, OAuth, and token operations. Use Notion from the terminal, scripts, or AI agents without opening the Notion app.

---

## Table of Contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Repository Structure](#repository-structure)
- [Commands Reference](#commands-reference)
  - **Commands:** [page](#page) | [block](#block) | [database](#database) | [datasource](#datasource) | [comment](#comment) | [user](#user) | [search](#search) | [file](#file) | [oauth](#oauth) | [token](#token)
- [Environment Variables](#environment-variables)
- [Testing Status](#testing-status)
- [Requirements](#requirements)
- [License](#license)

---

## Installation

### One-liner (requires Rust)

```bash
curl -fsSL https://raw.githubusercontent.com/Sankalpcreat/Notion-CLI/main/install.sh | sh
```

### From source

```bash
git clone https://github.com/Sankalpcreat/Notion-CLI.git
cd Notion-CLI
cargo build --release
sudo cp target/release/notion /usr/local/bin/
```

### Cargo install

```bash
cargo install cli-notion
```

Or install directly from GitHub:

```bash
cargo install --git https://github.com/Sankalpcreat/Notion-CLI.git --bin notion
```

**Requirements:** Rust 1.85+ ([rustup.rs](https://rustup.rs))

---

## Authentication

Set one of the following:

1. `NOTION_API_KEY` (recommended)
2. `NOTION_TOKEN`
3. `~/.notion/credentials.json` with:

```json
{"token":"secret_xxx"}
```

You can also keep local dev credentials in `.env` (already supported by `dotenvy`).

---

## Repository Structure

```text
Notion-CLI/
├── .env.example
├── README.md
├── AGENTS.md
├── Cargo.toml
├── install.sh
├── assets/
│   └── notion-cli-banner.png
└── src/
    ├── main.rs
    ├── client.rs
    ├── credentials.rs
    └── commands/
        ├── mod.rs
        ├── page.rs
        ├── block.rs
        ├── database.rs
        ├── datasource.rs
        ├── comment.rs
        ├── user.rs
        ├── search.rs
        ├── file.rs
        ├── oauth.rs
        └── token.rs
```

---

## Commands Reference

### page

| Command | Use |
|---------|-----|
| `page create --parent <id> [--title <text>]` | Create a page |
| `page get <page_id> [--filter-properties <id> ...]` | Retrieve a page |
| `page update <page_id> [--title ...] [--trash bool] [--icon ...] [--cover ...] [--lock bool] [--template bool] [--erase-content bool]` | Update page metadata/content flags |
| `page move <page_id> --parent <id>` | Move page to new parent |
| `page markdown <page_id>` | Retrieve page markdown |
| `page markdown-update <page_id> --operation <replace\|insert\|replace-range\|update> --content ...` | Update page markdown |
| `page property <page_id> <property_id> [--pagesize N] [--startcursor C]` | Get page property item |

### block

| Command | Use |
|---------|-----|
| `block get <block_id>` | Retrieve a block |
| `block children <block_id> [--pagesize N] [--startcursor C]` | List children |
| `block append <block_id> [--text ...] [--position start\|end\|after:<id>]` | Append child block |
| `block update <block_id> --text <text>` | Update paragraph text |
| `block delete <block_id>` | Delete block |

### database

| Command | Use |
|---------|-----|
| `database create --parent <page_id> [--title ...]` | Create database |
| `database get <database_id>` | Retrieve database |
| `database update <database_id> [--title ...] [--description ...] [--icon ...] [--cover ...]` | Update database |
| `database query <database_id> ...` | Legacy/deprecated query path |

### datasource

| Command | Use |
|---------|-----|
| `datasource create --parent <database_id> [--title ...]` | Create data source |
| `datasource get <data_source_id>` | Retrieve data source |
| `datasource update <data_source_id> [--title ...] [--icon ...] [--cover ...] [--trash bool] [--parent <id>] [--parent-type database_id\|page_id] [--properties <json>]` | Update data source |
| `datasource query <data_source_id> [--pagesize N] [--startcursor C] [--filter <json>] [--sorts <json>]` | Query data source |
| `datasource templates <data_source_id>` | List templates |

### comment

| Command | Use |
|---------|-----|
| `comment create --text <text> (--pageid <id> \| --blockid <id> \| --discussionid <id>)` | Create comment/reply |
| `comment list (--pageid <id> \| --blockid <id>) [--pagesize N] [--startcursor C]` | List comments |
| `comment get <comment_id>` | Get comment |

### user

| Command | Use |
|---------|-----|
| `user me` | Get current bot user |
| `user get <user_id>` | Get user |
| `user list [--pagesize N] [--startcursor C]` | List users |

### search

| Command | Use |
|---------|-----|
| `search [--query Q] [--filter page\|data_source] [--pagesize N] [--startcursor C]` | Search pages/data sources |

### file

| Command | Use |
|---------|-----|
| `file create --path <file> [--filename N] [--content-type M] [--mode single_part\|multi_part] [--parts N]` | Create file upload |
| `file list` | List file uploads |
| `file get <file_upload_id>` | Get upload status |
| `file send <file_upload_id> --path <file> [--part N]` | Send part (multipart flow) |
| `file complete <file_upload_id>` | Complete multipart upload |

### oauth

| Command | Use |
|---------|-----|
| `oauth token --code <code> --redirecturi <uri> [--clientid ...] [--clientsecret ...]` | Exchange code for token |
| `oauth refresh --refreshtoken <token> [--clientid ...] [--clientsecret ...]` | Refresh token |

### token

| Command | Use |
|---------|-----|
| `token introspect` | Introspect token |
| `token revoke` | Revoke token |

---

## Environment Variables

| Variable | Use |
|----------|-----|
| `NOTION_API_KEY` | Primary token for API calls |
| `NOTION_TOKEN` | Alternate token variable |
| `NOTION_CLIENT_ID` | OAuth client id |
| `NOTION_CLIENT_SECRET` | OAuth client secret |

---

## Testing Status

Current command surface: **37 commands**.

Latest full verification report after recent fixes:

- `OK`: 31
- `EXPECTED` (environment/plan/deprecated constraints): 6
- `FAIL`: 0

Examples of expected constraints:

1. `database query` uses a deprecated API path.
2. Multipart file completion depends on multipart eligibility/flow.
3. OAuth/token endpoints require correct OAuth credentials/token types.

---

## Requirements

- Rust (stable)
- Notion integration token (`secret_...`) or OAuth credentials for OAuth flows
- Access to pages/databases in the connected workspace

---

## License

MIT
