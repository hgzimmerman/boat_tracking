pub mod queries;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    diesel_derive_newtype::DieselNewType,
)]
pub struct UseScenarioId(i32);

impl UseScenarioId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
    pub fn as_int(&self) -> i32 {
        self.0
    }
}

impl std::str::FromStr for UseScenarioId {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(Self)
    }
}

impl std::fmt::Display for UseScenarioId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A use scenario loaded from the database.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable)]
#[diesel(table_name = crate::schema::use_scenario)]
pub struct UseScenario {
    pub id: UseScenarioId,
    pub name: String,
    pub default_time: Option<String>,
}

/// For inserting new scenarios.
#[derive(Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::schema::use_scenario)]
pub struct NewUseScenario {
    pub name: String,
    pub default_time: Option<String>,
}

/// For updating existing scenarios.
#[derive(Debug, Clone, diesel::AsChangeset)]
#[diesel(table_name = crate::schema::use_scenario)]
pub struct UseScenarioChangeset {
    pub name: Option<String>,
    pub default_time: Option<Option<String>>,
}
