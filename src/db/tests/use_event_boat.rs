use super::*;
use crate::db::boat::Boat;
use crate::db::use_event::{NewUseEvent, UseEvent};

/// Use events are returned newest-first when querying by boat.
#[test]
fn events_for_boat_returns_newest_first() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    let old_time = chrono::Utc::now() - chrono::TimeDelta::hours(2);
    let new_time = chrono::Utc::now();

    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: old_time,
            use_scenario: UseScenario::Adult,
            note: Some("Old".to_string()),
        },
    )
    .expect("should create old event");

    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: new_time,
            use_scenario: UseScenario::Adult,
            note: Some("New".to_string()),
        },
    )
    .expect("should create new event");

    let events = UseEvent::events_for_boat(&mut conn, boat.id)
        .expect("should get events");

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].note.as_deref(), Some("New"));
    assert_eq!(events[1].note.as_deref(), Some("Old"));
}

/// Events for one boat are not visible when querying a different boat.
#[test]
fn events_scoped_to_their_boat() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");

    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat_a.id,
            batch_id: None,
            recorded_at: chrono::Utc::now(),
            use_scenario: UseScenario::Adult,
            note: None,
        },
    )
    .expect("should create");

    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat_b.id,
            batch_id: None,
            recorded_at: chrono::Utc::now(),
            use_scenario: UseScenario::Regatta,
            note: None,
        },
    )
    .expect("should create");

    let alpha_events = UseEvent::events_for_boat(&mut conn, boat_a.id)
        .expect("should get events");
    let bravo_events = UseEvent::events_for_boat(&mut conn, boat_b.id)
        .expect("should get events");

    assert_eq!(alpha_events.len(), 1);
    assert_eq!(alpha_events[0].use_scenario, UseScenario::Adult);
    assert_eq!(bravo_events.len(), 1);
    assert_eq!(bravo_events[0].use_scenario, UseScenario::Regatta);
}

/// The daily timeseries fills in zero-count entries for days with no activity.
#[test]
fn daily_timeseries_includes_zero_days() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    let start = chrono::Utc::now() - chrono::TimeDelta::days(3);
    let end = chrono::Utc::now();

    // Create one event on day 1 only
    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: start + chrono::TimeDelta::hours(1),
            use_scenario: UseScenario::Adult,
            note: None,
        },
    )
    .expect("should create");

    let timeseries = UseEvent::daily_timeseries_for_boat(
        &mut conn,
        boat.id,
        start,
        Some(end),
    )
    .expect("should get timeseries");

    // Should have entries for each day in the range
    assert!(timeseries.len() >= 3);

    // Total uses across all days should be 1
    let total: usize = timeseries.iter().map(|(_, count)| count).sum();
    assert_eq!(total, 1);
}

/// CSV export joins boat metadata (name, type, weight class) onto each use event.
#[test]
fn export_events_joins_boat_data() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: chrono::Utc::now(),
            use_scenario: UseScenario::LearnToRow,
            note: None,
        },
    )
    .expect("should create");

    let rows = UseEvent::export_events(&mut conn, None, None, None)
        .expect("should export");

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].boat_name, "Alpha");
    assert_eq!(rows[0].use_scenario, UseScenario::LearnToRow);
    assert_eq!(rows[0].boat_id, boat.id);
}

/// CSV export can be filtered to specific boats by ID.
#[test]
fn export_events_filters_by_boat_ids() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");

    for boat in [&boat_a, &boat_b] {
        UseEvent::new_event(
            &mut conn,
            NewUseEvent {
                boat_id: boat.id,
                batch_id: None,
                recorded_at: chrono::Utc::now(),
                use_scenario: UseScenario::Adult,
                note: None,
            },
        )
        .expect("should create");
    }

    let rows = UseEvent::export_events(
        &mut conn,
        None,
        None,
        Some(vec![boat_a.id]),
    )
    .expect("should export");

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].boat_name, "Alpha");
}

/// CSV export respects date_start, excluding events before the cutoff.
#[test]
fn export_events_filters_by_date_range() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    let old_time = chrono::Utc::now() - chrono::TimeDelta::days(10);
    let recent_time = chrono::Utc::now();
    let cutoff = chrono::Utc::now() - chrono::TimeDelta::days(5);

    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: old_time,
            use_scenario: UseScenario::Adult,
            note: None,
        },
    )
    .expect("should create old");

    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: recent_time,
            use_scenario: UseScenario::Adult,
            note: None,
        },
    )
    .expect("should create recent");

    // Only events after the cutoff
    let rows = UseEvent::export_events(
        &mut conn,
        Some(cutoff),
        None,
        None,
    )
    .expect("should export");

    assert_eq!(rows.len(), 1);
}

/// Relinquished (retired) boats are excluded from filtered boat listings.
#[test]
fn relinquished_boat_excluded_from_filtered_list() {
    let mut conn = test_conn();
    let _boat_a = create_boat(&mut conn, "Active");
    let boat_b = create_boat(&mut conn, "Retired");

    Boat::get_rid_of_boat(&mut conn, boat_b.id).expect("should relinquish");

    let boats = Boat::get_filtered_boats(
        &mut conn,
        Default::default(),
        None,
    )
    .expect("should get boats");

    assert_eq!(boats.len(), 1);
    assert_eq!(boats[0].name, "Active");
}
