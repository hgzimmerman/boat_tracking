# Dioxus → HTMX 4.0 + Maud Migration Plan

**Project:** Boat Tracking Application
**Timeline:** 3-4 weeks (aggressive, full-time)
**Current:** Dioxus 0.5.x Fullstack (~3,708 lines UI code)
**Target:** HTMX 4.0 + Maud + Axum (single binary)

## Executive Summary

This migration eliminates the split web/SSR build complexity by moving to a pure server-rendered architecture with HTMX for interactivity. All existing functionality is preserved while achieving:

- **Single binary deployment** (no WASM)
- **Simplified builds** (no toolchain complexity)
- **Smaller client payload** (14KB HTMX vs WASM bundle)
- **Server-side state only** (no client signals/coroutines)
- **~50% less code overall**

## Architecture Decisions

Based on project requirements:

1. **Charts:** Server-side SVG generation with Plotters crate
   - No client-side JavaScript for charts
   - Smaller payload than Chart.js
   - Fast server-side rendering

2. **Dropdowns:** Alpine.js (12KB) for interactive UI
   - Weight class and boat type selectors
   - Instant feedback without server round-trips
   - Similar syntax to Vue.js

3. **State Management:** Hidden form fields (stateless)
   - Selected boats stored in hidden inputs
   - No server-side sessions needed
   - Simple and predictable

4. **Migration:** Aggressive 3-4 week full-time timeline
   - Replace entire system quickly
   - Test phase by phase
   - Deploy once validated

## Technology Stack Changes

### Add
```toml
[dependencies]
maud = "0.26"
plotters = { version = "0.3", default-features = false, features = ["svg_backend"] }
```

### Remove
```toml
# All Dioxus crates
dioxus = "0.5.1"
dioxus-router = "0.5.0"
dioxus-fullstack = "0.5.2"
dioxus-web = "0.5.0-alpha.2"
dioxus-charts = "0.2.0"

# WASM tooling
wasm-bindgen = "0.2.100"
wasm-bindgen-futures = "0.4.50"
web-sys = "0.3.66"
getrandom = { features = ["js"] }
tracing-web = "0.1.3"
```

### Keep
- axum, tokio (backend)
- diesel, deadpool-diesel (database)
- serde, chrono (data)
- Tailwind CSS (styling)

### Static Assets
- Download HTMX 4.0 to `public/htmx.min.js`
- Download Alpine.js to `public/alpine.min.js`

## New Directory Structure

```
src/
├── main.rs (single entry point, Axum setup)
├── lib.rs (minimal)
├── db/ (UNCHANGED - keep all database code)
├── templates/ (NEW - Maud templates)
│   ├── mod.rs
│   ├── layout.rs (base HTML, navigation, HTMX/Alpine includes)
│   ├── components/
│   │   ├── mod.rs
│   │   ├── modal.rs
│   │   ├── toast.rs
│   │   ├── dropdown.rs
│   │   └── forms.rs
│   ├── boats/
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   ├── detail.rs
│   │   ├── form.rs
│   │   └── charts.rs
│   ├── batches/
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   ├── creation.rs
│   │   ├── search_pane.rs
│   │   └── list_pane.rs
│   └── issues/
│       ├── mod.rs
│       ├── list.rs
│       └── form.rs
├── handlers/ (NEW - HTTP request handlers)
│   ├── mod.rs
│   ├── boats.rs
│   ├── batches.rs
│   └── issues.rs
└── state.rs (simplified AppState only)
```

## Migration Phases (18-22 days)

### Week 1: Foundation + Simple Pages

#### Phase 0: Foundation (2 days)
- Add Maud to Cargo.toml
- Download HTMX 4.0 and Alpine.js to public/
- Create `src/templates/layout.rs` with base HTML
- Create `src/handlers/mod.rs` structure
- Set up proof-of-concept page
- Verify HTMX and Alpine load correctly

**Files Created:**
- `src/templates/mod.rs`
- `src/templates/layout.rs`
- `src/templates/components/mod.rs`
- `src/handlers/mod.rs`
- `public/htmx.min.js`
- `public/alpine.min.js`

#### Phase 1: Simple Pages (3 days)
Migrate read-only pages without complex interactions:

1. **IssueListPage** - Simple table
2. **NewIssuePage** - Basic form
3. **BoatListPage** - List with stats
4. **Modal** - Hidden div component
5. **Toast** - Server-rendered notifications

**Pattern Established:** Handler fetches data → Maud template renders HTML → HTMX swaps

**Files Created:**
- `src/templates/issues/list.rs`
- `src/templates/issues/form.rs`
- `src/templates/boats/list.rs`
- `src/templates/components/modal.rs`
- `src/templates/components/toast.rs`
- `src/handlers/issues.rs`
- `src/handlers/boats.rs`

**Files to Remove After Testing:**
- `src/ui/components/issue_list.rs`
- `src/ui/components/new_issue.rs`
- `src/ui/components/boat_list.rs`
- `src/ui/components/modal.rs`

### Week 2: Forms + Charts

#### Phase 2: Forms with Validation (4 days)
Complex form handling with Alpine.js dropdowns:

1. **BoatForm** (create + edit modes)
   - Text inputs
   - Alpine.js dropdowns (weight class, boat type)
   - Date inputs
   - Server-side validation
   - Error display inline

**Alpine.js Dropdown Pattern:**
```html
<div x-data="{ open: false, selected: '' }">
  <button @click="open = !open" type="button">
    <span x-text="selected || 'Select...'"></span>
  </button>
  <div x-show="open" @click.away="open = false">
    <button @click="selected = 'Light'; open = false" type="button">
      Light
      <input type="radio" name="weight" value="light" x-model="selected" hidden>
    </button>
  </div>
</div>
```

**Files Created:**
- `src/templates/boats/form.rs` (large, ~200 lines)
- `src/templates/components/dropdown.rs`
- Update `src/handlers/boats.rs` (POST endpoints)

**Files to Remove:**
- `src/ui/components/boat/creation_edit_form/mod.rs` (323 lines)
- `src/ui/components/boat/creation_edit_form/service.rs`
- `src/ui/components/boat/creation_edit_form/new_boat_page.rs`
- `src/ui/components/boat/creation_edit_form/edit_tab.rs`

#### Phase 3: Charts with Plotters (3 days)
Server-side SVG chart generation:

1. **BoatMonthlyUses** - 30-day bar chart
2. **BoatYearlyUses** - 12-month bar chart

**Plotters Pattern:**
```rust
use plotters::prelude::*;

pub fn monthly_usage_chart(data: &[(NaiveDate, i64)]) -> String {
    let mut buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut buffer, (800, 400))
            .into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .build_cartesian_2d(
                data[0].0..data.last().unwrap().0,
                0..*data.iter().map(|(_, c)| c).max().unwrap()
            )
            .unwrap();

        chart.draw_series(
            data.iter().map(|(date, count)| {
                Rectangle::new([(*date, 0), (*date, *count)], BLUE.filled())
            })
        ).unwrap();
    }
    buffer
}
```

**Files Created:**
- `src/templates/boats/charts.rs`
- Update `src/handlers/boats.rs` (chart data endpoints)

**Files to Remove:**
- `src/ui/components/boat/use_count_chart_tabs.rs`

### Week 3: Complex Interactions

#### Phase 4: Navigation + Routing (2 days)
Full HTMX-based navigation:

1. **NavBar** - Top-level nav with hx-boost
2. **BoatListNav** - Section nav
3. **BoatNav** - Tab navigation for boat details

**HTMX Navigation Pattern:**
```html
<nav hx-boost="true" hx-target="#content">
  <a href="/boats" class="nav-link">Boats</a>
  <a href="/batches" class="nav-link active">Batches</a>
</nav>
```

**Files Modified:**
- `src/templates/layout.rs` (add full navigation)
- `src/templates/boats/detail.rs` (create boat detail layout)

#### Phase 5: Pagination (2 days)
Server-side pagination with HTMX:

**Pattern:**
```html
<a hx-get="/batches?page=2"
   hx-target="body"
   hx-swap="outerHTML"
   hx-push-url="true">Next</a>
```

**Files Created:**
- `src/templates/batches/list.rs`
- Update `src/handlers/batches.rs`

**Files to Remove:**
- `src/ui/components/batch_list.rs`

#### Phase 6: Toast Notifications (2 days)
HTMX Out-of-Band (OOB) swaps for server-triggered toasts:

**Pattern:**
```rust
// Any handler can return a toast
html! {
    div hx-redirect="/boats/123" {}  // Main response

    // OOB toast (inserted into #toast-container)
    div #toast-{unique_id} hx-swap-oob="afterbegin:#toast-container" {
        div .toast .success data-auto-dismiss="4000" {
            "Boat created successfully!"
        }
    }
}
```

**Client-side auto-dismiss (30 lines):**
```javascript
// public/toast.js
document.addEventListener('htmx:afterSwap', () => {
    document.querySelectorAll('[data-auto-dismiss]').forEach(toast => {
        const delay = parseInt(toast.dataset.autoDismiss);
        setTimeout(() => toast.remove(), delay);
    });
});
```

**Files Created:**
- `public/toast.js`
- Update `src/templates/components/toast.rs`

**Files to Remove:**
- `src/ui/components/toast.rs` (218 lines)

### Week 4: Two-Pane Interface + Cleanup

#### Phase 7: Two-Pane Batch Creation (5 days)
**MOST COMPLEX COMPONENT** - Search/filter/select boats:

**Architecture:**
- Left pane: Selected boats with remove buttons
- Right pane: Search results with add buttons
- State: Hidden form fields (boat IDs as hidden inputs)
- Synchronization: Server returns both panes on add/remove

**Flow:**
1. User searches → HTMX `hx-get="/api/boats/search?name=X"` → Update search pane
2. User clicks "Add" → HTMX `hx-post="/api/batch-session/add/{id}"` → Server re-renders both panes
3. Selected boats tracked in hidden inputs: `<input type="hidden" name="boat_ids[]" value="{id}">`
4. On submit → All boat IDs sent to server

**Search with Filters Pattern:**
```html
<input type="text"
       name="search"
       hx-get="/api/batch-session/search"
       hx-trigger="keyup changed delay:500ms"
       hx-target="#search-results"
       hx-include="[name='filter_oars'], [name='filter_coxed']">

<select name="filter_oars"
        hx-get="/api/batch-session/search"
        hx-trigger="change"
        hx-target="#search-results"
        hx-include="[name='search'], [name='filter_coxed']">
  <option value="">All</option>
  <option value="scull">Sculling</option>
  <option value="sweep">Sweep</option>
</select>
```

**Add Boat Handler:**
```rust
pub async fn add_boat_handler(
    State(state): State<AppState>,
    Path(boat_id): Path<BoatId>,
    Form(session): Form<BatchSessionState>,  // Contains boat_ids from hidden inputs
) -> Result<Html<String>, StatusCode> {
    // Parse existing selections
    let mut boat_ids: Vec<BoatId> = session.boat_ids
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Add new boat
    if !boat_ids.contains(&boat_id) {
        boat_ids.push(boat_id);
    }

    // Fetch boats to render
    let conn = state.conn().await?;
    let (selected, search_results) = conn.interact(move |conn| {
        let selected = Boat::get_by_ids(conn, &boat_ids)?;
        let exclude: HashSet<_> = boat_ids.iter().copied().collect();
        let search = Boat::get_filtered_boats(conn, filter, None)?
            .into_iter()
            .filter(|b| !exclude.contains(&b.id))
            .collect();
        Ok::<_, anyhow::Error>((selected, search))
    }).await??;

    // Return entire two-pane interface (both panes update)
    Ok(Html(templates::batches::batch_creation_page(
        &selected,
        &search_results,
        &filter,
    ).into_string()))
}
```

**Files Created:**
- `src/templates/batches/creation.rs` (large, ~250 lines)
- `src/templates/batches/search_pane.rs`
- `src/templates/batches/list_pane.rs`
- Update `src/handlers/batches.rs` (add, remove, search, create endpoints)

**Files to Remove:**
- `src/ui/components/batch/mod.rs` (217 lines)
- `src/ui/components/batch/service.rs` (290 lines - complex coroutine)
- `src/ui/components/batch/search_pane.rs`
- `src/ui/components/batch/list_pane/mod.rs`
- `src/ui/components/batch/list_pane/submit_row.rs`

#### Phase 8: Batch Modes (2 days)
Implement View, Edit, Template modes:

1. **BatchViewingPage** - Read-only
2. **BatchEditPage** - Modify existing
3. **BatchTemplateCreationPage** - Pre-populate from existing

**Pattern:** Same templates with mode parameter for conditional rendering

**Files Modified:**
- `src/templates/batches/creation.rs` (add mode handling)
- `src/handlers/batches.rs` (view, edit, template endpoints)

#### Phase 9: Hover Interactions (1 day)
Preview boats in batch on hover:

**Pattern:**
```html
<div hx-get="/api/batches/{id}/boats"
     hx-trigger="mouseenter once"
     hx-target="#tooltip-{id}"
     hx-swap="innerHTML">
  Hover to see boats
  <div id="tooltip-{id}"></div>
</div>
```

**Files Modified:**
- `src/templates/batches/list.rs`

#### Phase 10: Cleanup (2 days)
Remove old code, finalize:

1. Delete entire `src/ui/` directory (3,708 lines)
2. Remove Dioxus dependencies from Cargo.toml
3. Remove web/ssr features
4. Delete `dioxus.toml`
5. Simplify `src/main.rs` to single entry point
6. Update `README.md` with new build instructions
7. Full integration testing
8. Performance testing

**Files Deleted:**
- `src/ui/` (entire directory, 26 files)
- `dioxus.toml`

**Files Modified:**
- `Cargo.toml` (remove Dioxus deps, remove features)
- `src/main.rs` (single Axum server only)
- `src/lib.rs` (remove ui module)
- `README.md`

## Critical Implementation Files

These 5 files are the foundation - implement in this order:

### 1. `src/templates/layout.rs`
Base HTML structure for all pages:

```rust
use maud::{html, Markup, DOCTYPE, PreEscaped};

pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " - GGRC Boat Tracker" }
                link rel="stylesheet" href="/tailwind.css";
                script src="/htmx.min.js" {}
                script src="/alpine.min.js" defer {}
                script src="/toast.js" {}
            }
            body .bg-slate-50 .dark:bg-slate-500 {
                (navbar())
                div #content .flex .flex-col .flex-grow {
                    (content)
                }
                div #toast-container .fixed .top-8 .right-8 .space-y-2 {}
            }
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav #main-nav .bg-ggrc .sticky .px-4 .top-0 hx-boost="true" hx-target="#content" {
            ul .flex .items-center {
                li .mr-3 {
                    a .inline-block .border .rounded .py-2 .px-4
                      .bg-blue-500 .hover:bg-blue-700 .text-white
                      href="/batches" { "Practices and Regattas" }
                }
                li .mr-3 {
                    a .inline-block .border .rounded .py-2 .px-4
                      .bg-blue-500 .hover:bg-blue-700 .text-white
                      href="/boats" { "Boats" }
                }
            }
        }
    }
}
```

### 2. `src/templates/batches/creation.rs`
Most complex component - two-pane interface:

```rust
use maud::{html, Markup};
use crate::db::boat::{Boat, BoatFilter};

pub fn batch_creation_page(
    selected: &[Boat],
    search_results: &[Boat],
    filter: &BoatFilter,
) -> Markup {
    crate::templates::layout::page("Create Batch", html! {
        div #batch-creation .flex .flex-row .divide-x-4 .h-full {
            (selected_pane(selected))
            (search_pane(search_results, filter))
        }
    })
}

fn selected_pane(boats: &[Boat]) -> Markup {
    html! {
        div #selected-pane .w-1/2 .p-4 .overflow-y-auto {
            h2 .text-2xl .font-bold .mb-4 {
                "Selected Boats (" (boats.len()) ")"
            }

            form #batch-form hx-post="/api/batches/create" hx-target="body" {
                // Hidden inputs for each selected boat
                @for boat in boats {
                    input type="hidden" name="boat_ids[]" value=(boat.id.to_string());
                }

                div .divide-y .mb-4 {
                    @for boat in boats {
                        div .flex .justify-between .items-center .py-2 {
                            span { (boat.name) }
                            button type="button"
                                   hx-post=(format!("/api/batch-session/remove/{}", boat.id))
                                   hx-include="#batch-form"
                                   hx-target="#batch-creation"
                                   hx-swap="outerHTML"
                                   .btn-sm .text-red-600 {
                                "Remove"
                            }
                        }
                    }
                }

                @if !boats.is_empty() {
                    // Session type dropdown
                    select name="session_type" required .mb-2 .p-2 .border .rounded {
                        option value="" { "Select session type..." }
                        option value="practice" { "Practice" }
                        option value="regatta" { "Regatta" }
                    }

                    // Timestamp
                    input type="datetime-local" name="recorded_at" required
                          .mb-4 .p-2 .border .rounded;

                    button type="submit" .btn .btn-blue .w-full {
                        "Create Batch"
                    }
                }
            }
        }
    }
}

fn search_pane(boats: &[Boat], filter: &BoatFilter) -> Markup {
    html! {
        div #search-pane .w-1/2 .p-4 .overflow-y-auto {
            h2 .text-2xl .font-bold .mb-4 { "Search Boats" }

            // Search input with debouncing
            input type="text"
                  name="search"
                  placeholder="Search by name..."
                  .w-full .p-2 .border .rounded .mb-4
                  hx-get="/api/batch-session/search"
                  hx-trigger="keyup changed delay:500ms"
                  hx-target="#search-results"
                  hx-include="[name^='filter_']";

            // Filters
            div .flex .gap-2 .mb-4 {
                select name="filter_oars"
                       .p-2 .border .rounded
                       hx-get="/api/batch-session/search"
                       hx-trigger="change"
                       hx-target="#search-results"
                       hx-include="[name='search'], [name^='filter_']" {
                    option value="" { "All oar configs" }
                    option value="scull" selected[filter.oars_config.as_ref().map(|x| x.to_string()) == Some("scull".into())] { "Sculling" }
                    option value="sweep" selected[filter.oars_config.as_ref().map(|x| x.to_string()) == Some("sweep".into())] { "Sweep" }
                }

                select name="filter_coxed" .p-2 .border .rounded
                       hx-get="/api/batch-session/search"
                       hx-trigger="change"
                       hx-target="#search-results"
                       hx-include="[name='search'], [name^='filter_']" {
                    option value="" { "All" }
                    option value="coxed" { "Coxed" }
                    option value="coxless" { "Coxless" }
                }
            }

            // Results
            div #search-results {
                (search_results(boats))
            }
        }
    }
}

fn search_results(boats: &[Boat]) -> Markup {
    html! {
        div .divide-y {
            @if boats.is_empty() {
                p .text-gray-500 .text-center .py-4 { "No boats found" }
            } @else {
                @for boat in boats {
                    div .flex .justify-between .items-center .py-2 {
                        div {
                            div .font-medium { (boat.name) }
                            div .text-sm .text-gray-600 {
                                (boat.boat_type().unwrap_or_default().to_string())
                                " - "
                                (boat.weight_class.to_string())
                            }
                        }
                        button hx-post=(format!("/api/batch-session/add/{}", boat.id))
                               hx-include="#batch-form"
                               hx-target="#batch-creation"
                               hx-swap="outerHTML"
                               .btn-sm .bg-green-500 .text-white {
                            "Add"
                        }
                    }
                }
            }
        }
    }
}
```

### 3. `src/handlers/batches.rs`
Backend for batch creation logic:

```rust
use axum::{
    extract::{Path, Query, State, Form},
    response::{Html, Redirect, Response},
    http::StatusCode,
};
use serde::Deserialize;
use std::collections::HashSet;
use crate::{
    db::boat::{Boat, BoatFilter, BoatId},
    db::use_event_batch::BatchId,
    state::AppState,
    templates,
};

#[derive(Deserialize)]
pub struct SearchParams {
    search: Option<String>,
    filter_oars: Option<String>,
    filter_coxed: Option<String>,
}

#[derive(Deserialize)]
pub struct BatchSessionState {
    #[serde(default)]
    boat_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreateBatchForm {
    boat_ids: Vec<String>,
    session_type: String,
    recorded_at: String,
}

pub async fn batch_creation_page_handler(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    // Initial load: no selections, all boats
    let conn = state.conn().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let all_boats = conn
        .interact(|conn| Boat::get_filtered_boats(conn, BoatFilter::default(), None))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(templates::batches::batch_creation_page(
        &[],
        &all_boats,
        &BoatFilter::default(),
    ).into_string()))
}

pub async fn search_boats_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Html<String>, StatusCode> {
    let filter = BoatFilter {
        oars_config: params.filter_oars.and_then(|s| s.parse().ok()),
        coxed: params.filter_coxed.and_then(|s| s.parse().ok()),
        num_seats: None,
    };

    let conn = state.conn().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let boats = conn
        .interact(move |conn| Boat::get_filtered_boats(conn, filter, params.search))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return just the search results partial
    Ok(Html(templates::batches::search_results(&boats).into_string()))
}

pub async fn add_boat_handler(
    State(state): State<AppState>,
    Path(boat_id): Path<BoatId>,
    Form(session): Form<BatchSessionState>,
) -> Result<Html<String>, StatusCode> {
    // Parse existing selections from hidden inputs
    let mut boat_ids: Vec<BoatId> = session
        .boat_ids
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Add new boat if not already present
    if !boat_ids.contains(&boat_id) {
        boat_ids.push(boat_id);
    }

    // Fetch all boats
    let conn = state.conn().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (selected_boats, search_boats) = conn
        .interact(move |conn| {
            let selected = Boat::get_by_ids(conn, &boat_ids)?;
            let excluded: HashSet<_> = boat_ids.iter().copied().collect();
            let search = Boat::get_filtered_boats(conn, BoatFilter::default(), None)?
                .into_iter()
                .filter(|b| !excluded.contains(&b.id))
                .collect();
            Ok::<_, anyhow::Error>((selected, search))
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return entire two-pane interface (both panes update)
    Ok(Html(templates::batches::batch_creation_page(
        &selected_boats,
        &search_boats,
        &BoatFilter::default(),
    ).into_string()))
}

pub async fn remove_boat_handler(
    State(state): State<AppState>,
    Path(boat_id): Path<BoatId>,
    Form(session): Form<BatchSessionState>,
) -> Result<Html<String>, StatusCode> {
    // Remove boat from selections
    let boat_ids: Vec<BoatId> = session
        .boat_ids
        .iter()
        .filter_map(|s| s.parse().ok())
        .filter(|id| *id != boat_id)
        .collect();

    // Fetch boats (same as add_boat_handler)
    let conn = state.conn().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (selected_boats, search_boats) = conn
        .interact(move |conn| {
            let selected = Boat::get_by_ids(conn, &boat_ids)?;
            let excluded: HashSet<_> = boat_ids.iter().copied().collect();
            let search = Boat::get_filtered_boats(conn, BoatFilter::default(), None)?
                .into_iter()
                .filter(|b| !excluded.contains(&b.id))
                .collect();
            Ok::<_, anyhow::Error>((selected, search))
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(templates::batches::batch_creation_page(
        &selected_boats,
        &search_boats,
        &BoatFilter::default(),
    ).into_string()))
}

pub async fn create_batch_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateBatchForm>,
) -> Result<Response, StatusCode> {
    // Parse boat IDs
    let boat_ids: Vec<BoatId> = form
        .boat_ids
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    if boat_ids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Parse session type and timestamp
    let session_type = form.session_type.parse()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let recorded_at = chrono::NaiveDateTime::parse_from_str(&form.recorded_at, "%Y-%m-%dT%H:%M")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Create batch
    let conn = state.conn().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let batch_id = conn
        .interact(move |conn| {
            crate::db::use_event_batch::create_batch(
                conn,
                &boat_ids,
                session_type,
                recorded_at,
            )
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Redirect with success toast
    Ok((
        StatusCode::OK,
        Html(html! {
            div hx-redirect=(format!("/batches/{}", batch_id)) {}
            (templates::components::toast::success_toast("Batch created!"))
        }.into_string())
    ).into_response())
}
```

### 4. `src/templates/boats/form.rs`
Form with Alpine.js dropdowns:

```rust
use maud::{html, Markup};
use crate::db::boat::types::{BoatType, WeightClass};

pub struct BoatFormData {
    pub name: String,
    pub boat_type: Option<BoatType>,
    pub weight_class: Option<WeightClass>,
    pub acquired_at: Option<String>,
    pub manufactured_at: Option<String>,
    pub relinquished_at: Option<String>,
}

pub struct FormError {
    pub field: &'static str,
    pub message: String,
}

pub fn boat_form_page(
    data: Option<&BoatFormData>,
    errors: &[FormError],
    is_edit: bool,
    boat_id: Option<BoatId>,
) -> Markup {
    let title = if is_edit { "Edit Boat" } else { "New Boat" };
    let action = if is_edit {
        format!("/api/boats/{}/update", boat_id.unwrap())
    } else {
        "/api/boats/create".to_string()
    };

    crate::templates::layout::page(title, html! {
        form .max-w-2xl .mx-auto .bg-white .shadow-md .rounded .px-8 .py-6
             hx-post=(action)
             hx-target="body"
             x-data=r#"{
                 weightOpen: false,
                 typeOpen: false,
                 weightSelected: '',
                 typeSelected: ''
             }"# {

            h2 .text-3xl .font-bold .mb-6 { (title) }

            // Name field
            div .mb-4 {
                label .block .text-gray-700 .font-bold .mb-2 for="boat_name" {
                    "Boat Name"
                }
                input type="text"
                      name="name"
                      id="boat_name"
                      .w-full .p-2 .border .rounded
                      value=[data.map(|d| d.name.as_str())]
                      required;
                @if let Some(err) = errors.iter().find(|e| e.field == "name") {
                    span .text-red-500 .text-sm { (err.message) }
                }
            }

            // Weight class dropdown (Alpine.js)
            div .mb-4 {
                label .block .text-gray-700 .font-bold .mb-2 { "Weight Class" }
                div .relative x-data=r#"{ selected: '' }"# {
                    button type="button"
                           "@click"="weightOpen = !weightOpen"
                           .w-full .p-2 .border .rounded .text-left .bg-white {
                        span x-text=r#"weightSelected || 'Select weight class...'"# {}
                    }
                    div x-show="weightOpen"
                        "@click.away"="weightOpen = false"
                        .absolute .z-10 .w-full .bg-white .border .rounded .shadow-lg .mt-1 {
                        @for weight in &[WeightClass::Light, WeightClass::Medium, WeightClass::Heavy] {
                            button type="button"
                                   "@click"=(format!("weightSelected = '{}'; weightOpen = false", weight))
                                   .block .w-full .text-left .px-4 .py-2 .hover:bg-gray-100 {
                                (weight.to_string())
                            }
                        }
                    }
                    input type="hidden"
                          name="weight_class"
                          x-model="weightSelected"
                          required;
                }
                @if let Some(err) = errors.iter().find(|e| e.field == "weight_class") {
                    span .text-red-500 .text-sm { (err.message) }
                }
            }

            // Boat type dropdown (similar pattern)
            div .mb-4 {
                label .block .text-gray-700 .font-bold .mb-2 { "Boat Type" }
                div .relative {
                    button type="button"
                           "@click"="typeOpen = !typeOpen"
                           .w-full .p-2 .border .rounded .text-left .bg-white {
                        span x-text=r#"typeSelected || 'Select boat type...'"# {}
                    }
                    div x-show="typeOpen"
                        "@click.away"="typeOpen = false"
                        .absolute .z-10 .w-full .bg-white .border .rounded .shadow-lg .mt-1 .max-h-64 .overflow-y-auto {
                        @for boat_type in BoatType::all_variants() {
                            button type="button"
                                   "@click"=(format!("typeSelected = '{}'; typeOpen = false", boat_type))
                                   .block .w-full .text-left .px-4 .py-2 .hover:bg-gray-100 {
                                (boat_type.to_string())
                            }
                        }
                    }
                    input type="hidden" name="boat_type" x-model="typeSelected" required;
                }
                @if let Some(err) = errors.iter().find(|e| e.field == "boat_type") {
                    span .text-red-500 .text-sm { (err.message) }
                }
            }

            // Date fields
            div .mb-4 {
                label .block .text-gray-700 .font-bold .mb-2 for="acquired_at" {
                    "Acquired Date"
                }
                input type="date"
                      name="acquired_at"
                      id="acquired_at"
                      .w-full .p-2 .border .rounded
                      value=[data.and_then(|d| d.acquired_at.as_deref())];
            }

            div .mb-4 {
                label .block .text-gray-700 .font-bold .mb-2 for="manufactured_at" {
                    "Manufactured Date"
                }
                input type="date"
                      name="manufactured_at"
                      id="manufactured_at"
                      .w-full .p-2 .border .rounded
                      value=[data.and_then(|d| d.manufactured_at.as_deref())];
            }

            @if is_edit {
                div .mb-4 {
                    label .block .text-gray-700 .font-bold .mb-2 for="relinquished_at" {
                        "Relinquished Date"
                    }
                    input type="date"
                          name="relinquished_at"
                          id="relinquished_at"
                          .w-full .p-2 .border .rounded
                          value=[data.and_then(|d| d.relinquished_at.as_deref())];
                }
            }

            // Submit button
            button type="submit"
                   .w-full .bg-blue-500 .hover:bg-blue-700 .text-white .font-bold .py-2 .px-4 .rounded {
                @if is_edit { "Save Changes" } @else { "Create Boat" }
            }
        }
    })
}
```

### 5. `src/main.rs`
Single entry point with all routes:

```rust
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod db;
mod handlers;
mod state;
mod templates;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create database connection pool
    let state = state::AppState::new("db.sql");

    // Build router
    let app = Router::new()
        // Boats
        .route("/boats", get(handlers::boats::boat_list_handler))
        .route("/boats/new", get(handlers::boats::new_boat_page_handler))
        .route("/boats/:id", get(handlers::boats::boat_detail_handler))
        .route("/boats/:id/monthly", get(handlers::boats::boat_monthly_chart_handler))
        .route("/boats/:id/yearly", get(handlers::boats::boat_yearly_chart_handler))
        .route("/boats/:id/issues", get(handlers::boats::boat_issues_handler))
        .route("/boats/:id/edit", get(handlers::boats::edit_boat_page_handler))

        // Boat API
        .route("/api/boats/create", post(handlers::boats::create_boat_handler))
        .route("/api/boats/:id/update", post(handlers::boats::update_boat_handler))

        // Batches
        .route("/batches", get(handlers::batches::batch_list_handler))
        .route("/batches/new", get(handlers::batches::batch_creation_page_handler))
        .route("/batches/:id", get(handlers::batches::batch_view_handler))
        .route("/batches/edit/:id", get(handlers::batches::batch_edit_handler))

        // Batch API
        .route("/api/batch-session/search", get(handlers::batches::search_boats_handler))
        .route("/api/batch-session/add/:id", post(handlers::batches::add_boat_handler))
        .route("/api/batch-session/remove/:id", post(handlers::batches::remove_boat_handler))
        .route("/api/batches/create", post(handlers::batches::create_batch_handler))
        .route("/api/batches/:id/update", post(handlers::batches::update_batch_handler))

        // Issues
        .route("/issues", get(handlers::issues::issue_list_handler))
        .route("/issues/new", get(handlers::issues::new_issue_handler))
        .route("/api/issues/create", post(handlers::issues::create_issue_handler))

        // CSV Exports (existing)
        .route("/uses_export.csv", get(crate::api::export_uses_csv_handler))
        .route("/boats_export.csv", get(crate::api::export_boats_csv_handler))

        // Static files
        .nest_service("/", ServeDir::new("public"))

        // State
        .with_state(state);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

## Key Patterns Reference

### HTMX Patterns

**Search with Debouncing:**
```html
<input hx-get="/search"
       hx-trigger="keyup changed delay:500ms"
       hx-target="#results">
```

**Form Submission:**
```html
<form hx-post="/create"
      hx-target="body"
      hx-swap="outerHTML">
```

**Navigation:**
```html
<nav hx-boost="true" hx-target="#content">
  <a href="/boats">Boats</a>
</nav>
```

**Out-of-Band Swap (Toasts):**
```html
<div hx-swap-oob="afterbegin:#toast-container">
  <div class="toast">Success!</div>
</div>
```

**Include Other Form Fields:**
```html
<select hx-get="/filter"
        hx-include="[name='search'], [name='other_filter']">
```

### Alpine.js Dropdown Pattern

```html
<div x-data="{ open: false, selected: '' }">
  <button @click="open = !open">
    <span x-text="selected || 'Select...'"></span>
  </button>
  <div x-show="open" @click.away="open = false">
    <button @click="selected = 'Option 1'; open = false">
      Option 1
      <input type="hidden" name="field" x-model="selected">
    </button>
  </div>
</div>
```

### Plotters Chart Pattern

```rust
use plotters::prelude::*;

pub fn generate_bar_chart(data: &[(NaiveDate, i64)]) -> String {
    let mut buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut buffer, (800, 400))
            .into_drawing_area();
        root.fill(&WHITE).unwrap();

        let max_val = data.iter().map(|(_, v)| v).max().unwrap_or(&10);

        let mut chart = ChartBuilder::on(&root)
            .caption("Usage Over Time", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(
                data[0].0..data.last().unwrap().0,
                0..*max_val
            )
            .unwrap();

        chart
            .configure_mesh()
            .draw()
            .unwrap();

        chart.draw_series(
            data.iter().map(|(date, count)| {
                let x0 = *date;
                let x1 = *date + chrono::Duration::days(1);
                Rectangle::new([(x0, 0), (x1, *count)], BLUE.filled())
            })
        ).unwrap();
    }
    buffer
}
```

## Testing Checklist

After each phase:

- [ ] All pages load without errors
- [ ] Forms submit successfully
- [ ] Validation errors display correctly
- [ ] Navigation works (back button, bookmarks)
- [ ] Toasts appear and auto-dismiss
- [ ] Charts render correctly
- [ ] Search/filter updates results
- [ ] Add/remove boats updates both panes
- [ ] Batch creation saves correctly
- [ ] CSV exports still work
- [ ] Tailwind styles apply correctly
- [ ] No console errors (check browser DevTools)
- [ ] HTMX requests complete (check Network tab)

## Success Criteria

Migration is complete when:

1. All 15 routes work with HTMX navigation
2. All forms submit and validate correctly
3. Charts display usage data as SVG
4. Two-pane batch creation works smoothly
5. Toasts display on all actions
6. CSV exports unchanged and working
7. No Dioxus code remains
8. Single `cargo build --release` produces deployable binary
9. Binary size <50MB (vs >100MB with WASM)
10. Build time <2 minutes (vs >5 minutes with WASM)

## Risk Mitigation

**Data Loss in Forms:**
- Use `hx-include` to preserve form state across requests
- Test partially-filled forms thoroughly
- Alpine.js handles dropdown state locally (no server round-trip)

**Chart Performance:**
- Cache generated SVGs with proper HTTP headers
- Plotters is fast for small datasets (<1000 points)
- Consider lazy loading charts (HTMX trigger on viewport)

**State Management:**
- Hidden form fields prevent server-side session complexity
- Document that batch selections lost on refresh (acceptable for this use case)
- If needed later, add tower-sessions for persistence

**Testing:**
- Test each phase before moving to next
- Keep Dioxus version running until HTMX fully validated
- Use browser DevTools Network tab to debug HTMX requests

## Next Steps

1. Review this plan
2. Add dependencies to Cargo.toml
3. Start Phase 0 (Foundation)
4. Work through phases sequentially
5. Test thoroughly at each phase
6. Deploy once Phase 10 complete

The migration path is clear, incremental, and testable. With aggressive timeline (3-4 weeks full-time), this is achievable by implementing 1-2 phases per week.

## TODO: Outstanding Features

### Batch Page Advanced Search
The current batch creation page (Phase 7) includes basic filters for oars configuration and coxed/coxless, but is missing:

- [ ] **Weight Class Filter** - Dropdown to filter boats by Light/Medium/Heavy weight class
- [ ] **Boat Type Filter** - Dropdown to filter boats by specific boat type (1x, 2-, 4+, 8+, etc.)

These filters should:
1. Be added to the search pane in `src/templates/batches/creation.rs` alongside existing filters
2. Follow the same HTMX pattern as oars_config filter (line 600-610 in this plan)
3. Include all filter values in search requests using `hx-include="[name='search'], [name^='filter_']"`
4. Update `BoatFilter` in database queries to support weight_class and boat_type filtering
5. Be implemented in `src/handlers/batches.rs` search_boats_handler

**Implementation Pattern:**
```html
<select name="filter_weight"
       class="p-2 border rounded"
       hx-get="/api/batch-session/search"
       hx-trigger="change"
       hx-target="#search-results"
       hx-include="[name='search'], [name^='filter_']">
    <option value="">All Weight Classes</option>
    <option value="light">Light</option>
    <option value="medium">Medium</option>
    <option value="heavy">Heavy</option>
</select>

<select name="filter_boat_type"
       class="p-2 border rounded"
       hx-get="/api/batch-session/search"
       hx-trigger="change"
       hx-target="#search-results"
       hx-include="[name='search'], [name^='filter_']">
    <option value="">All Boat Types</option>
    <option value="1x">1x</option>
    <option value="2-">2-</option>
    <option value="4+">4+</option>
    <option value="8+">8+</option>
    <!-- etc for all boat types -->
</select>
```
