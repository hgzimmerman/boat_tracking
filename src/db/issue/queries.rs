use super::*;
use diesel::SqliteConnection;
use crate::schema::issue;

impl Issue {
    pub fn add_issue(
        conn: &mut SqliteConnection,
        new: NewIssue,
    ) -> Result<Issue, diesel::result::Error> {
        diesel::insert_into(crate::schema::issue::table)
            .values(new)
            .get_result(conn)
    }

    pub fn get_open_issues_for_boat(
        conn: &mut SqliteConnection,
        boat_id: BoatId,
    ) -> Result<Vec<Issue>, diesel::result::Error> {
        issue::table
            .filter(issue::boat_id.eq(boat_id).and(issue::resolved_at.is_null()))
            .order_by(issue::recorded_at.desc())
            .get_results(conn)
    }
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
}