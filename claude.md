# Boat Tracking System - Project Context

## Project Overview

Boat inventory and tracking system for GGRC (rowing club). Locally-hosted server for tracking boat usage, maintenance issues, and fleet management.

**Primary Use Case**: A computer in the boathouse for quick data entry after practices/regattas.

## Architecture

### Tech Stack

- **Language**: Rust (Edition 2021)
- **Server**: Axum 0.8
- **Frontend**: Server-rendered HTML via HTMX + Maud templates
- **Desktop**: Tauri 2 (optional, `--features tauri`) wraps the server in a native window
- **Database**: SQLite via Diesel 2.3 with deadpool connection pooling
- **Styling**: Tailwind CSS v3 (standalone CLI, no Node.js)
- **Logging**: tracing with non-blocking rolling file appender
- **Dev Environment**: Nix flakes

### Project Structure

```
src/
├── api/            # CSV export endpoints
├── db/             # Database layer (Diesel models + queries)
│   ├── boat/       # Boat models, types, and queries
│   ├── issue/      # Issue tracking
│   ├── use_event/  # Individual boat use records
│   ├── use_event_batch/ # Batch operations for sessions
│   └── state.rs    # AppState and connection pool
├── handlers/       # Axum route handlers (HTMX endpoints)
├── templates/      # Maud HTML templates
│   ├── boats/
│   ├── batches/
│   ├── issues/
│   ├── components/ # Shared UI components (modals, toasts, dropdowns)
│   └── layout.rs
├── schema.rs       # Diesel schema (auto-generated)
├── lib.rs
└── main.rs

migrations/         # Diesel migrations (embedded, run automatically on startup)
public/             # Static assets (htmx.min.js, alpine.min.js, tailwind.css)
```

### Cargo Features

- `tauri`: Builds as a Tauri desktop app (spawns Axum in background, opens native window)
- Default (no features): Runs as a standalone Axum server on port 3000

### Database

- **Engine**: SQLite (file-based at `db.sql`)
- **ORM**: Diesel with embedded migrations (run automatically on startup)
- **Connection Pooling**: deadpool-diesel
- **All query functions** are instrumented with `tracing::instrument` for logging

### CI/CD

- **PR checks**: Linux + Windows builds, clippy with `-D warnings`
- **Releases**: Full builds with artifact uploads on GitHub release publish

## Common Patterns

- All db types derive `Debug`
- `tracing::instrument` on queries: `skip(conn)` for writes, `skip_all` for reads, `err` on all
- `serde` for serialization, `chrono` for dates, `anyhow`/`thiserror` for errors
- Templates return `maud::Markup`, handlers return `axum::response::Response`
