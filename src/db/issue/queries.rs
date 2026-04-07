use super::*;
use crate::{db::{DbOrdering, boat::Boat}, schema::{issue, boat}};
use diesel::SqliteConnection;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

impl Issue {
    #[tracing::instrument(level = "debug", skip(conn), err)]
    pub fn add_issue(
        conn: &mut SqliteConnection,
        new: NewIssue,
    ) -> Result<Issue, diesel::result::Error> {
        diesel::insert_into(crate::schema::issue::table)
            .values(new)
            .get_result(conn)
    }

    #[tracing::instrument(level = "debug", skip(conn), err)]
    pub fn resolve_issue(
        conn: &mut SqliteConnection,
        issue_id: IssueId,
    ) -> Result<Issue, diesel::result::Error> {
        let now = chrono::Utc::now();
        diesel::update(issue::table.filter(issue::id.eq(issue_id)))
            .set(issue::resolved_at.eq(Some(now)))
            .get_result(conn)
    }

    #[tracing::instrument(level = "debug", skip(conn), err)]
    pub fn unresolve_issue(
        conn: &mut SqliteConnection,
        issue_id: IssueId,
    ) -> Result<Issue, diesel::result::Error> {
        diesel::update(issue::table.filter(issue::id.eq(issue_id)))
            .set(issue::resolved_at.eq(None::<chrono::DateTime<chrono::Utc>>))
            .get_result(conn)
    }

    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn get_open_issues_for_boat(
        conn: &mut SqliteConnection,
        boat_id: BoatId,
    ) -> Result<Vec<Issue>, diesel::result::Error> {
        issue::table
            .filter(issue::boat_id.eq(boat_id).and(issue::resolved_at.is_null()))
            .order_by(issue::recorded_at.desc())
            .get_results(conn)
    }
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn get_resolved_issues_for_boat(
        conn: &mut SqliteConnection,
        boat_id: BoatId,
    ) -> Result<Vec<Issue>, diesel::result::Error> {
        issue::table
            .filter(
                issue::boat_id
                    .eq(boat_id)
                    .and(issue::resolved_at.is_not_null()),
            )
            .order_by(issue::recorded_at.desc())
            .get_results(conn)
    }
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn get_all_unresolved_issues(
        conn: &mut SqliteConnection,
        order: DbOrdering,
    ) -> Result<Vec<Issue>, diesel::result::Error> {
        match order {
            DbOrdering::Asc => issue::table
                .filter(issue::resolved_at.is_not_null())
                .order_by(issue::recorded_at.asc())
                .get_results(conn),
            DbOrdering::Desc => issue::table
                .filter(issue::resolved_at.is_not_null())
                .order_by(issue::recorded_at.desc())
                .get_results(conn),
        }
    }
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn get_all_issues(
        conn: &mut SqliteConnection,
        order: DbOrdering,
    ) -> Result<Vec<Issue>, diesel::result::Error> {
        match order {
            DbOrdering::Asc => issue::table
                .order_by(issue::recorded_at.asc())
                .then_order_by(issue::resolved_at.asc())
                .get_results(conn),
            DbOrdering::Desc => issue::table
                .order_by(issue::recorded_at.desc())
                .then_order_by(issue::resolved_at.desc())
                .get_results(conn),
        }
    }

    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn get_all_issues_with_boats(
        conn: &mut SqliteConnection,
        order: DbOrdering,
    ) -> Result<Vec<(Issue, Option<Boat>)>, diesel::result::Error> {
        use diesel::SelectableHelper;

        match order {
            DbOrdering::Asc => issue::table
                .left_outer_join(boat::table)
                .select((Issue::as_select(), Option::<Boat>::as_select()))
                .order_by(issue::recorded_at.asc())
                .then_order_by(issue::resolved_at.asc())
                .load(conn),
            DbOrdering::Desc => issue::table
                .left_outer_join(boat::table)
                .select((Issue::as_select(), Option::<Boat>::as_select()))
                .order_by(issue::recorded_at.desc())
                .then_order_by(issue::resolved_at.desc())
                .load(conn),
        }
    }
}
