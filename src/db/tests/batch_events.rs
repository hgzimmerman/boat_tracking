use super::*;
use crate::db::use_event::UseEvent;
use crate::db::use_event_batch::UseEventBatch;

#[test]
fn create_batch_creates_use_events_for_each_boat() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");

    let batch_id = create_batch(
        &mut conn,
        vec![boat_a.id, boat_b.id],
        UseScenario::Adult,
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
        assert_eq!(event.use_scenario, UseScenario::Adult);
    }
}

#[test]
fn batch_appears_in_recent_list_with_correct_count() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");
    let boat_c = create_boat(&mut conn, "Charlie");

    let _batch_id = create_batch(
        &mut conn,
        vec![boat_a.id, boat_b.id, boat_c.id],
        UseScenario::Regatta,
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
    assert_eq!(batches[0].batch.use_scenario, UseScenario::Regatta);
}

#[test]
fn filter_batches_by_scenario() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    create_batch(&mut conn, vec![boat.id], UseScenario::Adult);
    create_batch(&mut conn, vec![boat.id], UseScenario::Regatta);
    create_batch(&mut conn, vec![boat.id], UseScenario::Adult);

    let adult_batches = UseEventBatch::get_most_recent_batches_and_their_use_count(
        &mut conn,
        Some(UseScenario::Adult),
        0,
        100,
    )
    .expect("should list batches");

    assert_eq!(adult_batches.len(), 2);
    for b in &adult_batches {
        assert_eq!(b.batch.use_scenario, UseScenario::Adult);
    }
}

#[test]
fn replace_batch_uses_swaps_boats() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");
    let boat_c = create_boat(&mut conn, "Charlie");

    let batch_id = create_batch(
        &mut conn,
        vec![boat_a.id, boat_b.id],
        UseScenario::Adult,
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

#[test]
fn replace_batch_uses_can_update_scenario() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    let batch_id = create_batch(&mut conn, vec![boat.id], UseScenario::Adult);

    UseEventBatch::replace_batch_uses(
        &mut conn,
        batch_id,
        vec![boat.id],
        Some(UseScenario::Regatta),
        None,
    )
    .expect("should replace");

    let batch = UseEventBatch::get_batch(&mut conn, batch_id)
        .expect("should get batch")
        .expect("batch should exist");
    assert_eq!(batch.use_scenario, UseScenario::Regatta);

    // The new events should also have the updated scenario
    let events = UseEventBatch::get_events_and_boats_for_batch(&mut conn, batch_id)
        .expect("should get events");
    for (event, _) in &events {
        assert_eq!(event.use_scenario, UseScenario::Regatta);
    }
}

#[test]
fn get_batch_returns_none_for_missing_id() {
    let mut conn = test_conn();

    let result = UseEventBatch::get_batch(&mut conn, BatchId::new(9999))
        .expect("query should succeed");
    assert!(result.is_none());
}

#[test]
fn events_for_boat_includes_batch_events() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    // Create a batch event and a standalone event
    create_batch(&mut conn, vec![boat.id], UseScenario::Adult);
    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: chrono::Utc::now(),
            use_scenario: UseScenario::Other,
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
