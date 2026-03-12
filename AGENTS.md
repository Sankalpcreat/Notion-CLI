# Agent Guide — notion-cli

Read this file first when you clone this repository or open it in an LLM agent.

---

## What This Project Is

`notion-cli` is a Rust command-line client for the Notion API.

- Current command surface: **37 commands**
- Focus: pages, blocks, databases/data sources, comments, users, search, file uploads, OAuth, token operations
- Binary name: `notion`

This is an independent project and not an official Notion product.

---

## Quick Start (for Agents)

1. Build:
```bash
cargo build --release
```

2. Auth: set one of
- `NOTION_API_KEY` (recommended)
- `NOTION_TOKEN`
- `~/.notion/credentials.json` containing `{"token":"..."}`

3. Run:
```bash
./target/release/notion --help
```

---

## Repository Map

| Path | Purpose |
|---|---|
| `README.md` | User-facing docs |
| `AGENTS.md` | Agent onboarding (this file) |
| `Cargo.toml` | Package + dependencies |
| `src/main.rs` | Entry point |
| `src/client.rs` | HTTP client (GET/POST/PATCH/DELETE/multipart) |
| `src/credentials.rs` | Token resolution from env/file |
| `src/commands/mod.rs` | CLI router |
| `src/commands/*.rs` | Command implementations |
| `assets/notion-cli-banner.png` | README banner |

---

## Architecture

- Clap builds top-level command groups in `src/commands/mod.rs`.
- Command handlers map CLI args to Notion REST endpoints.
- `src/client.rs` centralizes request headers:
  - `Authorization: Bearer <token>`
  - `Notion-Version: 2026-03-11`
- Credentials load order:
  1. `NOTION_API_KEY`
  2. `NOTION_TOKEN`
  3. `~/.notion/credentials.json`

---

## Full Command Surface (37)

### `page` (7)
1. `page create`
2. `page get`
3. `page update`
4. `page move`
5. `page markdown`
6. `page markdown-update`
7. `page property`

### `block` (5)
8. `block get`
9. `block children`
10. `block append`
11. `block update`
12. `block delete`

### `database` (4)
13. `database create`
14. `database get`
15. `database update`
16. `database query` (legacy/deprecated endpoint behavior)

### `datasource` (5)
17. `datasource create`
18. `datasource get`
19. `datasource update`
20. `datasource query`
21. `datasource templates`

### `comment` (3)
22. `comment create`
23. `comment list`
24. `comment get`

### `user` (3)
25. `user me`
26. `user get`
27. `user list`

### `search` (1)
28. `search`

### `file` (5)
29. `file create`
30. `file list`
31. `file get`
32. `file send`
33. `file complete`

### `oauth` (2)
34. `oauth token`
35. `oauth refresh`

### `token` (2)
36. `token introspect`
37. `token revoke`

---

## Common Agent Workflows

### Basic connectivity check
```bash
notion user me
```

### Find a page and inspect it
```bash
notion search --query "notes" --filter page --pagesize 10
notion page get <page_id>
```

### Create + update content
```bash
notion page create --parent <parent_page_id> --title "New Page"
notion block append <page_or_block_id> --text "Hello from CLI"
```

### Data source query
```bash
notion datasource query <data_source_id> --pagesize 20
```

---

## Known Constraints (Important)

- `database query` is a legacy/deprecated path; prefer `datasource query`.
- Multipart file upload behavior depends on workspace plan and API eligibility.
- OAuth commands require valid OAuth client credentials.
- `token introspect/revoke` semantics depend on token type and API constraints.

---

## Security Hygiene

Before committing:

1. Do not commit `.env`.
2. Do not commit real tokens/keys in docs or examples.
3. Keep example credentials in `.env.example` only.
4. Sanitize logs/reports before publishing.

---

## Suggested First Read Order for LLMs

1. `README.md`
2. `src/commands/mod.rs`
3. `src/client.rs`
4. `src/credentials.rs`
5. `src/commands/page.rs` and `src/commands/datasource.rs` (largest behavior surface)
