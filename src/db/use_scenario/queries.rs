use super::*;
use crate::schema::use_scenario;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SqliteConnection};

impl UseScenario {
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<UseScenario>, diesel::result::Error> {
        use_scenario::table
            .order_by(use_scenario::id.asc())
            .get_results(conn)
    }

    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn get_by_id(
        conn: &mut SqliteConnection,
        id: UseScenarioId,
    ) -> Result<Option<UseScenario>, diesel::result::Error> {
        use_scenario::table
            .filter(use_scenario::id.eq(id))
            .get_result(conn)
            .optional()
    }

    #[tracing::instrument(level = "debug", skip(conn), err)]
    pub fn create(
        conn: &mut SqliteConnection,
        new: NewUseScenario,
    ) -> Result<UseScenario, diesel::result::Error> {
        diesel::insert_into(use_scenario::table)
            .values(&new)
            .get_result(conn)
    }

    #[tracing::instrument(level = "debug", skip(conn), err)]
    pub fn update(
        conn: &mut SqliteConnection,
        id: UseScenarioId,
        changeset: UseScenarioChangeset,
    ) -> Result<UseScenario, diesel::result::Error> {
        diesel::update(use_scenario::table.filter(use_scenario::id.eq(id)))
            .set(&changeset)
            .get_result(conn)
    }
}
