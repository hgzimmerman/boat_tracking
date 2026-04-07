use super::*;
use crate::db::boat::BoatAndStats;
use crate::db::issue::Issue;
use crate::db::use_event::{NewUseEvent, UseEvent};

/// A boat with no issues or use events reports all stat counters as zero.
#[test]
fn boat_stats_zero_when_no_activity() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    let stats = BoatAndStats::get_boat(&mut conn, boat.id)
        .expect("should get stats");

    assert_eq!(stats.boat.id, boat.id);
    assert_eq!(stats.open_issues.unwrap_or(0), 0);
    assert_eq!(stats.total_uses.unwrap_or(0), 0);
    assert_eq!(stats.uses_last_thirty_days.unwrap_or(0), 0);
}

/// Only unresolved issues are counted in the open_issues stat; resolved ones are excluded.
#[test]
fn boat_stats_counts_open_issues_only() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    create_issue(&mut conn, boat.id, "Open issue 1");
    let issue_b = create_issue(&mut conn, boat.id, "Will resolve");
    create_issue(&mut conn, boat.id, "Open issue 2");

    Issue::resolve_issue(&mut conn, issue_b.id).expect("should resolve");

    let stats = BoatAndStats::get_boat(&mut conn, boat.id)
        .expect("should get stats");

    // Only 2 open issues (the resolved one shouldn't count)
    assert_eq!(stats.open_issues.unwrap_or(0), 2);
}

/// total_uses reflects every use event recorded for the boat.
#[test]
fn boat_stats_counts_total_uses() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    // Create some use events
    for _ in 0..5 {
        UseEvent::new_event(
            &mut conn,
            NewUseEvent {
                boat_id: boat.id,
                batch_id: None,
                recorded_at: chrono::Utc::now(),
                use_scenario_id: masters_am_scenario_id(),
                note: None,
            },
        )
        .expect("should create event");
    }

    let stats = BoatAndStats::get_boat(&mut conn, boat.id)
        .expect("should get stats");

    assert_eq!(stats.total_uses.unwrap_or(0), 5);
    assert_eq!(stats.uses_last_thirty_days.unwrap_or(0), 5);
}

/// uses_last_thirty_days excludes events older than 30 days while total_uses includes them.
#[test]
fn boat_stats_last_thirty_days_excludes_old_events() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    // Recent event
    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: chrono::Utc::now(),
            use_scenario_id: masters_am_scenario_id(),
            note: None,
        },
    )
    .expect("should create recent event");

    // Old event (60 days ago)
    let old_time = chrono::Utc::now() - chrono::TimeDelta::days(60);
    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat.id,
            batch_id: None,
            recorded_at: old_time,
            use_scenario_id: masters_am_scenario_id(),
            note: None,
        },
    )
    .expect("should create old event");

    let stats = BoatAndStats::get_boat(&mut conn, boat.id)
        .expect("should get stats");

    assert_eq!(stats.total_uses.unwrap_or(0), 2);
    assert_eq!(stats.uses_last_thirty_days.unwrap_or(0), 1);
}

/// The all-boats stats query returns one entry per boat, including boats with no activity.
#[test]
fn get_boats_returns_stats_for_all_boats() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");

    // Give Alpha some activity
    UseEvent::new_event(
        &mut conn,
        NewUseEvent {
            boat_id: boat_a.id,
            batch_id: None,
            recorded_at: chrono::Utc::now(),
            use_scenario_id: masters_am_scenario_id(),
            note: None,
        },
    )
    .expect("should create event");
    create_issue(&mut conn, boat_a.id, "Alpha issue");

    let all_stats = BoatAndStats::get_boats(&mut conn)
        .expect("should get all stats");

    assert_eq!(all_stats.len(), 2);

    let alpha = all_stats.iter().find(|s| s.boat.id == boat_a.id).unwrap();
    let bravo = all_stats.iter().find(|s| s.boat.id == boat_b.id).unwrap();

    assert!(alpha.total_uses.unwrap_or(0) >= 1);
    assert!(alpha.open_issues.unwrap_or(0) >= 1);
    assert_eq!(bravo.total_uses.unwrap_or(0), 0);
    assert_eq!(bravo.open_issues.unwrap_or(0), 0);
}

/// Use events created via a batch are counted in the boat's stats.
#[test]
fn batch_events_count_toward_boat_stats() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    // Create a batch (which internally creates use events)
    create_batch(&mut conn, vec![boat.id], regatta_scenario_id());

    let stats = BoatAndStats::get_boat(&mut conn, boat.id)
        .expect("should get stats");

    assert_eq!(stats.total_uses.unwrap_or(0), 1);
    assert_eq!(stats.uses_last_thirty_days.unwrap_or(0), 1);
}
