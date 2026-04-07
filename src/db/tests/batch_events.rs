use super::*;
use crate::db::use_event::UseEvent;
use crate::db::use_event_batch::UseEventBatch;

/// Creating a batch produces one use event per boat, each linked back to the batch.
#[test]
fn create_batch_creates_use_events_for_each_boat() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");

    let batch_id = create_batch(
        &mut conn,
        vec![boat_a.id, boat_b.id],
        masters_am_scenario_id(),
    );

    let events_and_boats =
        UseEventBatch::get_events_and_boats_for_batch(&mut conn, batch_id)
            .expect("should get events");

    assert_eq!(events_and_boats.len(), 2);

    let boat_names: Vec<&str> = events_and_boats
        .iter()
        .map(|(_, boat)| boat.name.as_str())
        .collect();
    assert!(boat_names.contains(&"Alpha"));
    assert!(boat_names.contains(&"Bravo"));

    // All events should reference the batch
    for (event, _) in &events_and_boats {
        assert_eq!(event.batch_id, Some(batch_id));
        assert_eq!(event.use_scenario_id, masters_am_scenario_id());
    }
}

/// The batch listing reports the correct number of boats used in each batch.
#[test]
fn batch_appears_in_recent_list_with_correct_count() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");
    let boat_c = create_boat(&mut conn, "Charlie");

    let _batch_id = create_batch(
        &mut conn,
        vec![boat_a.id, boat_b.id, boat_c.id],
        regatta_scenario_id(),
    );

    let batches = UseEventBatch::get_most_recent_batches_and_their_use_count(
        &mut conn,
        None,
        0,
        100,
    )
    .expect("should list batches");

    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].use_counts, 3);
    assert_eq!(batches[0].batch.use_scenario_id, regatta_scenario_id());
}

/// Batch listing can be narrowed to a single use scenario.
#[test]
fn filter_batches_by_scenario() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    create_batch(&mut conn, vec![boat.id], masters_am_scenario_id());
    create_batch(&mut conn, vec![boat.id], regatta_scenario_id());
    create_batch(&mut conn, vec![boat.id], masters_am_scenario_id());

    let adult_batches = UseEventBatch::get_most_recent_batches_and_their_use_count(
        &mut conn,
        Some(masters_am_scenario_id()),
        0,
        100,
    )
    .expect("should list batches");

    assert_eq!(adult_batches.len(), 2);
    for b in &adult_batches {
        assert_eq!(b.batch.use_scenario_id, masters_am_scenario_id());
    }
}

/// Replacing a batch's uses deletes the old events and creates new ones for the new set of boats.
#[test]
fn replace_batch_uses_swaps_boats() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");
    let boat_c = create_boat(&mut conn, "Charlie");

    let batch_id = create_batch(
        &mut conn,
        vec![boat_a.id, boat_b.id],
        masters_am_scenario_id(),
    );

    // Replace with a different set of boats
    UseEventBatch::replace_batch_uses(
        &mut conn,
        batch_id,
        vec![boat_b.id, boat_c.id],
        None,
        None,
    )
    .expect("should replace batch uses");

    let events_and_boats =
        UseEventBatch::get_events_and_boats_for_batch(&mut conn, batch_id)
            .expect("should get events");

    assert_eq!(events_and_boats.len(), 2);
    let boat_names: Vec<&str> = events_and_boats
        .iter()
        .map(|(_, b)| b.name.as_str())
        .collect();
    assert!(boat_names.contains(&"Bravo"));
    assert!(boat_names.contains(&"Charlie"));
    assert!(!boat_names.contains(&"Alpha"));
}

/// Replacing batch uses with a new scenario updates both the batch and its child events.
#[test]
fn replace_batch_uses_can_update_scenario() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    let batch_id = create_batch(&mut conn, vec![boat.id], masters_am_scenario_id());

    UseEventBatch::replace_batch_uses(
        &mut conn,
        batch_id,
        vec![boat.id],
        Some(regatta_scenario_id()),
        None,
    )
    .expect("should replace");

    let batch = UseEventBatch::get_batch(&mut conn, batch_id)
        .expect("should get batch")
        .expect("batch should exist");
    assert_eq!(batch.use_scenario_id, regatta_scenario_id());

    // The new events should also have the updated scenario
    let events = UseEventBatch::get_events_and_boats_for_batch(&mut conn, batch_id)
        .expect("should get events");
    for (event, _) in &events {
        assert_eq!(event.use_scenario_id, regatta_scenario_id());
    }
}

/// Looking up a batch that doesn't exist returns None rather than an error.
#[test]
fn get_batch_returns_none_for_missing_id() {
    let mut conn = test_conn();

    let result = UseEventBatch::get_batch(&mut conn, BatchId::new(9999))
        .expect("query should succeed");
    assert!(result.is_none());
}

/// Querying events for a boat returns both batch-created and standalone events.
#[test]
fn events_for_boat_includes_batch_events() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    // Create a batch event and a standalone event
    create_batch(&mut conn, vec![boat.id], masters_am_scenario_id());
    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: chrono::Utc::now(),
            use_scenario_id: masters_pm_scenario_id(),
            note: None,
        },
    )
    .expect("should create standalone event");

    let events = UseEvent::events_for_boat(&mut conn, boat.id)
        .expect("should get events");

    assert_eq!(events.len(), 2);
    // One has a batch_id, one doesn't
    let with_batch = events.iter().filter(|e| e.batch_id.is_some()).count();
    let without_batch = events.iter().filter(|e| e.batch_id.is_none()).count();
    assert_eq!(with_batch, 1);
    assert_eq!(without_batch, 1);
}
