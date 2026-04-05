# Boat Tracking System - Project Context

## Project Overview

This is a boat inventory and tracking system for GGRC (rowing club). It's implemented as a locally-hosted server that serves a single-page application for tracking boat usage, maintenance issues, and providing data-driven insights for boat fleet management.

**Primary Use Case**: A computer in the boathouse for quick data entry after practices/regattas to track which boats were used, identify maintenance needs, and inform decisions about buying, selling, or maintaining boats.

## Architecture

### Tech Stack

- **Language**: Rust (Edition 2021)
- **Frontend Framework**: Dioxus 0.5.1 (full-stack web framework)
- **Backend Framework**: Axum 0.7.0 (HTTP server)
- **Database**: SQLite with Diesel ORM 2.1.0
- **Styling**: Tailwind CSS
- **Build System**: Nix (flakes) for reproducible builds and dependency management

### Project Structure

```
src/
├── api/           # Backend API endpoints (Axum routes)
├── db/            # Database layer
│   ├── boat/      # Boat models and queries
│   ├── issue/     # Issue tracking models and queries
│   ├── use_event/ # Individual boat use records
│   └── use_event_batch/ # Batch operations for practice/regatta sessions
├── ui/            # Frontend layer (Dioxus components)
│   ├── components/ # Reusable UI components
│   └── util/      # Utility functions (time, loadable states, etc.)
└── main.rs        # Application entry point

migrations/        # Diesel database migrations
public/           # Static assets (compiled CSS, etc.)
```

## Key Features

1. **Boat Management**
   - Add, edit, and remove boats from inventory
   - Track boat metadata and history

2. **Usage Tracking**
   - Record batches of boat uses per practice/regatta
   - Edit historical batches
   - Use old batches as templates for faster data entry

3. **Issue Tracking** (WIP)
   - Report and track boat maintenance issues
   - Replaces traditional paper logbook

4. **Data Export**
   - CSV export of usage summaries
   - Per-boat usage history
   - Complete fleet usage history

## Development Workflow

### Build Features

The project uses Cargo feature flags to separate concerns:

- `ssr`: Server-side rendering features (Axum, Diesel with SQLite)
- `web`: Web/WASM features (Dioxus web, wasm-bindgen)

### Development Commands (via Nix shell)

- `watch-tailwind`: Watches for Tailwind class usage and regenerates CSS
- `watch-dx`: Hot-reloads frontend (web) changes
- `watch-server`: Hot-reloads backend (SSR) changes
- `run-server`: Builds and runs the server once

### Database

- **Engine**: SQLite (file-based at `db.sql`)
- **ORM**: Diesel with migrations in `migrations/`
- **Connection Pooling**: deadpool-diesel

### Environment Setup

1. Uses Nix flakes for reproducible development environment
2. `.envrc` with direnv for automatic environment loading
3. VS Code is the primary IDE with extensions:
   - rust-analyzer
   - Dioxus
   - Tailwind CSS IntelliSense
   - direnv

## Important Conventions

### Code Organization

- Database logic separated into modules by entity type (boat, issue, use_event, use_event_batch)
- Each DB module typically has:
  - `types.rs`: Model definitions and Diesel schema
  - `queries.rs`: Database query functions
  - `mod.rs`: Module exports

- UI components mirror the feature structure
- Shared utilities in `ui/util/`

### Data Flow

1. **Frontend** (Dioxus components in `src/ui/`)
   - Renders UI and handles user interactions
   - Makes requests to backend API

2. **API Layer** (`src/api/`)
   - Axum routes handling HTTP requests
   - Validates input and coordinates business logic

3. **Database Layer** (`src/db/`)
   - Diesel queries and models
   - Direct SQLite access via connection pool

### Common Patterns

- Uses `serde` for serialization throughout
- `chrono` for date/time handling
- `anyhow` for error handling in application code
- `thiserror` for custom error types
- `tracing` for logging/debugging

## Building for Production

The Nix flake includes a `defaultPackage` that:
1. Builds the backend with SSR features
2. Bundles the web frontend using Dioxus CLI (`dx bundle`)
3. Creates a deployable artifact

## Testing & Development Notes

- Run all three watch commands in separate terminals for best development experience
- Tailwind CSS needs to be regenerated when adding new utility classes
- Database migrations should be run via `diesel migration run`
- The project targets local deployment (single-user, local network)

## License

GNU GPL 3.0 - All contributions must be licensed under the same terms
