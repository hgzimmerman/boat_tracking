use super::*;
use crate::db::issue::Issue;
use crate::db::DbOrdering;

/// A new issue records its boat association and starts unresolved.
#[test]
fn issue_linked_to_boat() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");
    let issue = create_issue(&mut conn, boat.id, "Hull crack");

    assert_eq!(issue.boat_id, Some(boat.id));
    assert_eq!(issue.note, "Hull crack");
    assert!(issue.resolved_at.is_none());
}

/// Resolving sets a timestamp; unresolving clears it back to None.
#[test]
fn resolve_and_unresolve_issue() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");
    let issue = create_issue(&mut conn, boat.id, "Oarlock loose");

    // Resolve
    let resolved = Issue::resolve_issue(&mut conn, issue.id)
        .expect("should resolve");
    assert!(resolved.resolved_at.is_some());

    // Unresolve
    let unresolved = Issue::unresolve_issue(&mut conn, resolved.id)
        .expect("should unresolve");
    assert!(unresolved.resolved_at.is_none());
}

/// The open-issues query omits issues that have been resolved.
#[test]
fn open_issues_for_boat_excludes_resolved() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");
    let issue_a = create_issue(&mut conn, boat.id, "Open issue");
    let issue_b = create_issue(&mut conn, boat.id, "Will resolve");

    Issue::resolve_issue(&mut conn, issue_b.id).expect("should resolve");

    let open = Issue::get_open_issues_for_boat(&mut conn, boat.id)
        .expect("should get open issues");
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].id, issue_a.id);
}

/// The resolved-issues query omits issues that are still open.
#[test]
fn resolved_issues_for_boat_excludes_open() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");
    let _open = create_issue(&mut conn, boat.id, "Still open");
    let will_resolve = create_issue(&mut conn, boat.id, "Will resolve");

    let resolved_issue = Issue::resolve_issue(&mut conn, will_resolve.id).expect("should resolve");

    let resolved = Issue::get_resolved_issues_for_boat(&mut conn, boat.id)
        .expect("should get resolved issues");
    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].id, resolved_issue.id);
}

/// Issues for one boat are not visible when querying another boat.
#[test]
fn issues_scoped_to_their_boat() {
    let mut conn = test_conn();
    let boat_a = create_boat(&mut conn, "Alpha");
    let boat_b = create_boat(&mut conn, "Bravo");

    create_issue(&mut conn, boat_a.id, "Alpha issue 1");
    create_issue(&mut conn, boat_a.id, "Alpha issue 2");
    create_issue(&mut conn, boat_b.id, "Bravo issue");

    let alpha_issues = Issue::get_open_issues_for_boat(&mut conn, boat_a.id)
        .expect("should get issues");
    let bravo_issues = Issue::get_open_issues_for_boat(&mut conn, boat_b.id)
        .expect("should get issues");

    assert_eq!(alpha_issues.len(), 2);
    assert_eq!(bravo_issues.len(), 1);
}

/// The issues-with-boats join returns boat data for linked issues and None for orphan issues.
#[test]
fn get_all_issues_with_boats_joins_correctly() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");
    create_issue(&mut conn, boat.id, "Linked issue");

    // Also create an issue with no boat
    Issue::add_issue(
        &mut conn,
        NewIssue {
            boat_id: None,
            use_event_id: None,
            recorded_at: chrono::Utc::now(),
            note: "Orphan issue".to_string(),
            resolved_at: None,
        },
    )
    .expect("should create orphan issue");

    let all = Issue::get_all_issues_with_boats(&mut conn, DbOrdering::Desc, 0, 100)
        .expect("should get issues with boats");

    assert_eq!(all.len(), 2);

    let linked: Vec<_> = all.iter().filter(|(_, b)| b.is_some()).collect();
    let orphan: Vec<_> = all.iter().filter(|(_, b)| b.is_none()).collect();

    assert_eq!(linked.len(), 1);
    assert_eq!(linked[0].0.note, "Linked issue");
    assert_eq!(linked[0].1.as_ref().unwrap().name, "Alpha");

    assert_eq!(orphan.len(), 1);
    assert_eq!(orphan[0].0.note, "Orphan issue");
}

/// Issue listing respects the requested sort direction.
#[test]
fn get_all_issues_ordering() {
    let mut conn = test_conn();
    let boat = create_boat(&mut conn, "Alpha");

    // Create issues with slight time gaps
    let issue_a = create_issue(&mut conn, boat.id, "First");
    let issue_b = create_issue(&mut conn, boat.id, "Second");

    let asc = Issue::get_all_issues(&mut conn, DbOrdering::Asc)
        .expect("should get issues");
    assert_eq!(asc[0].id, issue_a.id);

    let desc = Issue::get_all_issues(&mut conn, DbOrdering::Desc)
        .expect("should get issues");
    assert_eq!(desc[0].id, issue_b.id);
}
