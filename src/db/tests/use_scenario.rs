use super::*;
use chrono::NaiveTime;
use crate::db::use_scenario::{NewUseScenario, UseScenario, UseScenarioChangeset};

/// The migration seeds 8 scenarios into a fresh database.
#[test]
fn migration_seeds_scenarios() {
    let mut conn = test_conn();
    let scenarios = UseScenario::get_all(&mut conn).expect("should get all");
    assert_eq!(scenarios.len(), 8);
}

/// Seeded scenarios preserve their expected names and ordering.
#[test]
fn seeded_scenarios_have_expected_names() {
    let mut conn = test_conn();
    let scenarios = UseScenario::get_all(&mut conn).expect("should get all");
    let names: Vec<&str> = scenarios.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, vec![
        "Youth GGRC Practice",
        "Youth Somerville Practice",
        "Masters AM Practice",
        "Masters PM Practice",
        "Learn To Row",
        "Sculling Saturday",
        "Private Session",
        "Regatta",
    ]);
}

/// Seeded scenarios with default times round-trip through the database correctly.
#[test]
fn seeded_default_times_round_trip() {
    let mut conn = test_conn();
    let am = UseScenario::get_by_id(&mut conn, masters_am_scenario_id())
        .expect("should query")
        .expect("should exist");
    assert_eq!(am.default_time, Some(NaiveTime::from_hms_opt(5, 30, 0).unwrap()));

    let pm = UseScenario::get_by_id(&mut conn, masters_pm_scenario_id())
        .expect("should query")
        .expect("should exist");
    assert_eq!(pm.default_time, Some(NaiveTime::from_hms_opt(18, 30, 0).unwrap()));
}

/// Scenarios without a default time store None.
#[test]
fn scenario_without_default_time_is_none() {
    let mut conn = test_conn();
    let regatta = UseScenario::get_by_id(&mut conn, regatta_scenario_id())
        .expect("should query")
        .expect("should exist");
    assert_eq!(regatta.default_time, None);
}

/// Looking up a nonexistent scenario returns None.
#[test]
fn get_by_id_returns_none_for_missing() {
    let mut conn = test_conn();
    let result = UseScenario::get_by_id(&mut conn, UseScenarioId::new(9999))
        .expect("query should succeed");
    assert!(result.is_none());
}

/// A new scenario can be created with a default time.
#[test]
fn create_scenario_with_default_time() {
    let mut conn = test_conn();
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    let created = UseScenario::create(
        &mut conn,
        NewUseScenario {
            name: "Midnight Row".to_string(),
            default_time: Some(midnight),
        },
    )
    .expect("should create");

    assert_eq!(created.name, "Midnight Row");
    assert_eq!(created.default_time, Some(midnight));

    // Verify it round-trips through a fresh read
    let fetched = UseScenario::get_by_id(&mut conn, created.id)
        .expect("should query")
        .expect("should exist");
    assert_eq!(fetched.default_time, Some(midnight));
}

/// A new scenario can be created without a default time.
#[test]
fn create_scenario_without_default_time() {
    let mut conn = test_conn();
    let created = UseScenario::create(
        &mut conn,
        NewUseScenario {
            name: "No Time".to_string(),
            default_time: None,
        },
    )
    .expect("should create");

    assert_eq!(created.default_time, None);
}

/// A scenario's name and default time can be updated.
#[test]
fn update_scenario_name_and_time() {
    let mut conn = test_conn();
    let new_time = NaiveTime::from_hms_opt(23, 59, 0).unwrap();

    let updated = UseScenario::update(
        &mut conn,
        regatta_scenario_id(),
        UseScenarioChangeset {
            name: Some("Championship Regatta".to_string()),
            default_time: Some(Some(new_time)),
        },
    )
    .expect("should update");

    assert_eq!(updated.name, "Championship Regatta");
    assert_eq!(updated.default_time, Some(new_time));
}

/// A scenario's default time can be cleared by setting it to None.
#[test]
fn clear_default_time() {
    let mut conn = test_conn();

    // Masters AM starts with a default time
    let before = UseScenario::get_by_id(&mut conn, masters_am_scenario_id())
        .expect("should query")
        .expect("should exist");
    assert!(before.default_time.is_some());

    let updated = UseScenario::update(
        &mut conn,
        masters_am_scenario_id(),
        UseScenarioChangeset {
            name: None,
            default_time: Some(None),
        },
    )
    .expect("should update");

    assert_eq!(updated.name, "Masters AM Practice"); // unchanged
    assert_eq!(updated.default_time, None);
}

/// Omitting default_time from the changeset leaves it unchanged.
#[test]
fn update_name_only_preserves_default_time() {
    let mut conn = test_conn();

    let original = UseScenario::get_by_id(&mut conn, masters_am_scenario_id())
        .expect("should query")
        .expect("should exist");

    let updated = UseScenario::update(
        &mut conn,
        masters_am_scenario_id(),
        UseScenarioChangeset {
            name: Some("Renamed".to_string()),
            default_time: None, // None means "don't change"
        },
    )
    .expect("should update");

    assert_eq!(updated.name, "Renamed");
    assert_eq!(updated.default_time, original.default_time);
}

/// Midnight (00:00) and end-of-day (23:59) times are stored and retrieved correctly.
#[test]
fn boundary_times_round_trip() {
    let mut conn = test_conn();
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let end_of_day = NaiveTime::from_hms_opt(23, 59, 0).unwrap();

    let s1 = UseScenario::create(
        &mut conn,
        NewUseScenario {
            name: "Midnight".to_string(),
            default_time: Some(midnight),
        },
    )
    .expect("should create");

    let s2 = UseScenario::create(
        &mut conn,
        NewUseScenario {
            name: "End of Day".to_string(),
            default_time: Some(end_of_day),
        },
    )
    .expect("should create");

    let fetched1 = UseScenario::get_by_id(&mut conn, s1.id)
        .expect("should query").expect("should exist");
    let fetched2 = UseScenario::get_by_id(&mut conn, s2.id)
        .expect("should query").expect("should exist");

    assert_eq!(fetched1.default_time, Some(midnight));
    assert_eq!(fetched2.default_time, Some(end_of_day));
}

/// Creating a scenario with a duplicate name fails.
#[test]
fn duplicate_name_fails() {
    let mut conn = test_conn();
    let result = UseScenario::create(
        &mut conn,
        NewUseScenario {
            name: "Youth GGRC Practice".to_string(), // already seeded
            default_time: None,
        },
    );
    assert!(result.is_err());
}

/// Newly created scenarios appear in get_all results.
#[test]
fn created_scenarios_appear_in_get_all() {
    let mut conn = test_conn();
    let before = UseScenario::get_all(&mut conn).expect("should get all").len();

    UseScenario::create(
        &mut conn,
        NewUseScenario {
            name: "Custom Scenario".to_string(),
            default_time: None,
        },
    )
    .expect("should create");

    let after = UseScenario::get_all(&mut conn).expect("should get all").len();
    assert_eq!(after, before + 1);
}
