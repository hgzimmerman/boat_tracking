// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    boat (id) {
        id -> Integer,
        name -> Text,
        weight_class -> WeightClassMapping,
        seat_count -> Integer,
        has_cox -> Integer,
        oars_per_seat -> Integer,
        acquired_at -> Nullable<Date>,
        manufactured_at -> Nullable<Date>,
        relinquished_at -> Nullable<Date>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    issue (id) {
        id -> Integer,
        boat_id -> Nullable<Integer>,
        use_event_id -> Nullable<Integer>,
        recorded_at -> Timestamp,
        note -> Text,
        resolved_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    use_event (id) {
        id -> Integer,
        boat_id -> Integer,
        batch_id -> Nullable<Integer>,
        recorded_at -> Timestamp,
        use_scenario -> UseScenarioMapping,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    use_event_batch (id) {
        id -> Integer,
        recorded_at -> Timestamp,
        use_scenario -> UseScenarioMapping,
    }
}

diesel::joinable!(issue -> boat (boat_id));
diesel::joinable!(issue -> use_event (use_event_id));
diesel::joinable!(use_event -> boat (boat_id));
diesel::joinable!(use_event -> use_event_batch(batch_id));

diesel::allow_tables_to_appear_in_same_query!(
    boat,
    issue,
    use_event,
    use_event_batch,
);
