use diesel::{Connection, SqliteConnection};
use diesel_migrations::MigrationHarness;

use crate::db::boat::types::{BoatId, BoatType, WeightClass};
use crate::db::boat::{Boat, NewBoat};
use crate::db::issue::NewIssue;
use crate::db::use_event::{NewUseEvent, UseScenario};
use crate::db::use_event_batch::{BatchId, NewBatch, NewBatchArgs};

mod batch_events;
mod boat_stats;
mod issue_boat;
mod use_event_boat;

/// Creates an in-memory SQLite connection with migrations applied.
///
/// Each call gets its own isolated database instance.
fn test_conn() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:")
        .expect("should connect to in-memory SQLite");
    conn.run_pending_migrations(crate::db::state::MIGRATIONS)
        .expect("should run migrations");
    conn
}

/// Helper to insert a default boat and return it.
fn create_boat(conn: &mut SqliteConnection, name: &str) -> Boat {
    Boat::new_boat(
        conn,
        NewBoat::new(
            name.to_string(),
            WeightClass::Medium,
            BoatType::Single,
            None,
            None,
        ),
    )
    .expect("should create boat")
}

/// Helper to insert a batch with the given boats. Returns the batch ID.
fn create_batch(
    conn: &mut SqliteConnection,
    boat_ids: Vec<BoatId>,
    scenario: UseScenario,
) -> BatchId {
    use crate::db::use_event_batch::UseEventBatch;
    UseEventBatch::create_batch(
        conn,
        NewBatchArgs {
            boat_ids,
            batch: NewBatch {
                use_scenario: scenario,
                recorded_at: chrono::Utc::now(),
            },
        },
    )
    .expect("should create batch")
}

/// Helper to insert an issue for a boat.
fn create_issue(conn: &mut SqliteConnection, boat_id: BoatId, note: &str) -> crate::db::issue::Issue {
    crate::db::issue::Issue::add_issue(
        conn,
        NewIssue {
            boat_id: Some(boat_id),
            use_event_id: None,
            recorded_at: chrono::Utc::now(),
            note: note.to_string(),
            resolved_at: None,
        },
    )
    .expect("should create issue")
}
