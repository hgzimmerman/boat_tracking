use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use super::{boat::types::BoatId, use_event::UseEventId, DbOrdering};
use crate::schema::issue;

#[derive(Debug, Clone, diesel::Identifiable, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::issue)]
pub struct Issue {
    pub id: IssueId,
    pub boat_id: Option<BoatId>, 
    pub use_event_id: Option<UseEventId>,
    pub recorded_at: chrono::NaiveDateTime,
    pub note: String,
    pub resolved_at: Option<chrono::NaiveDateTime>
}

#[derive(Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::schema::issue)]
pub struct NewIssue {
    pub boat_id: Option<BoatId>,
    pub use_event_id: Option<UseEventId>,
    pub recorded_at: chrono::NaiveDateTime,
    pub note: String,
    pub resolved_at: Option<chrono::NaiveDateTime>
}


impl Issue {
    pub fn add_issue(conn: &mut SqliteConnection, new: NewIssue) -> Result<Issue, diesel::result::Error> {
        diesel::insert_into(crate::schema::issue::table)  
        .values(new)
        .get_result(conn)
    }

    pub fn get_open_issues_for_boat(conn: &mut SqliteConnection, boat_id: BoatId) -> Result<Vec<Issue>, diesel::result::Error> {
        issue::table.filter(
            issue::boat_id.eq(boat_id).and(issue::resolved_at.is_null())
        )
        .order_by(issue::recorded_at.desc())
        .get_results(conn)
    }
    pub fn get_resolved_issues_for_boat(conn: &mut SqliteConnection, boat_id: BoatId) -> Result<Vec<Issue>, diesel::result::Error> {
        issue::table.filter(
            issue::boat_id.eq(boat_id).and(issue::resolved_at.is_not_null())
        )
        .order_by(issue::recorded_at.desc())
        .get_results(conn)
    }
    pub fn get_all_unresolved_issues(conn: &mut SqliteConnection, order: DbOrdering) -> Result<Vec<Issue>, diesel::result::Error> {
        match order {
            DbOrdering::Asc => {
                issue::table.filter(
                    issue::resolved_at.is_not_null()
                )
                .order_by(issue::recorded_at.asc())
                .get_results(conn)
            } ,
            DbOrdering::Desc => {
                issue::table.filter(
                    issue::resolved_at.is_not_null()
                )
                .order_by(issue::recorded_at.desc())
                .get_results(conn)
            }
        }
    }
    pub fn get_all_issues(conn: &mut SqliteConnection, order: DbOrdering) -> Result<Vec<Issue>, diesel::result::Error> {
        match order {
            DbOrdering::Asc => {
                issue::table
                .order_by(issue::recorded_at.asc())
                .then_order_by(issue::resolved_at.asc())
                .get_results(conn)
            } ,
            DbOrdering::Desc => {
                issue::table
                .order_by(issue::recorded_at.desc())
                .then_order_by(issue::resolved_at.desc())
                .get_results(conn)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize, diesel_derive_newtype::DieselNewType)]
pub struct IssueId(i32);

impl IssueId {
    pub fn new(new:i32) -> Self {
        Self(new)
    }
    pub fn as_int(&self) -> i32 {
        self.0
    }
}