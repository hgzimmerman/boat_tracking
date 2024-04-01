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
pub struct BoatId(i32);

impl BoatId {
    pub fn new(new: i32) -> Self {
        Self(new)
    }
    pub fn as_int(&self) -> i32 {
        self.0
    }
}
impl std::str::FromStr for BoatId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(Self)
    }
}
impl std::fmt::Display for BoatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum BoatType {
    Single,
    Double,
    DoublePlus,
    Quad,
    QuadPlus,
    Octo,
    OctoPlus,
    Pair,
    PairPlus,
    Four,
    FourPlus,
    Eight,
}

impl BoatType {
    pub fn into_values(self) -> (HasCox, SeatCount, OarsPerSeat) {
        match self {
            BoatType::Single => (HasCox(0), SeatCount(1), OarsPerSeat(2)),
            BoatType::Double => (HasCox(0), SeatCount(2), OarsPerSeat(2)),
            BoatType::DoublePlus => (HasCox(1), SeatCount(2), OarsPerSeat(2)),
            BoatType::Quad => (HasCox(0), SeatCount(4), OarsPerSeat(2)),
            BoatType::QuadPlus => (HasCox(1), SeatCount(4), OarsPerSeat(2)),
            BoatType::Octo => (HasCox(0), SeatCount(8), OarsPerSeat(2)),
            BoatType::OctoPlus => (HasCox(1), SeatCount(8), OarsPerSeat(2)),
            BoatType::Pair => (HasCox(0), SeatCount(2), OarsPerSeat(1)),
            BoatType::PairPlus => (HasCox(1), SeatCount(2), OarsPerSeat(1)),
            BoatType::Four => (HasCox(0), SeatCount(4), OarsPerSeat(1)),
            BoatType::FourPlus => (HasCox(1), SeatCount(4), OarsPerSeat(1)),
            BoatType::Eight => (HasCox(1), SeatCount(8), OarsPerSeat(1)),
        }
    }
    pub fn from_boat_attributes(attributes: BoatAttributes) -> Option<Self> {
        // let BoatAttributes { has_cox, seats, oar_configuation } = attributes;
        match attributes {
            BoatAttributes {
                has_cox: HasCox(0),
                seats: SeatCount(1),
                oar_configuation: OarsPerSeat(2),
            } => Some(BoatType::Single),
            BoatAttributes {
                has_cox: HasCox(0),
                seats: SeatCount(2),
                oar_configuation: OarsPerSeat(2),
            } => Some(BoatType::Double),
            BoatAttributes {
                has_cox: HasCox(1),
                seats: SeatCount(2),
                oar_configuation: OarsPerSeat(2),
            } => Some(BoatType::DoublePlus),
            BoatAttributes {
                has_cox: HasCox(0),
                seats: SeatCount(4),
                oar_configuation: OarsPerSeat(2),
            } => Some(BoatType::Quad),
            BoatAttributes {
                has_cox: HasCox(1),
                seats: SeatCount(4),
                oar_configuation: OarsPerSeat(2),
            } => Some(BoatType::QuadPlus),
            BoatAttributes {
                has_cox: HasCox(0),
                seats: SeatCount(8),
                oar_configuation: OarsPerSeat(2),
            } => Some(BoatType::Octo),
            BoatAttributes {
                has_cox: HasCox(1),
                seats: SeatCount(8),
                oar_configuation: OarsPerSeat(2),
            } => Some(BoatType::OctoPlus),
            BoatAttributes {
                has_cox: HasCox(0),
                seats: SeatCount(2),
                oar_configuation: OarsPerSeat(1),
            } => Some(BoatType::Pair),
            BoatAttributes {
                has_cox: HasCox(1),
                seats: SeatCount(2),
                oar_configuation: OarsPerSeat(1),
            } => Some(BoatType::PairPlus),
            BoatAttributes {
                has_cox: HasCox(0),
                seats: SeatCount(4),
                oar_configuation: OarsPerSeat(1),
            } => Some(BoatType::Four),
            BoatAttributes {
                has_cox: HasCox(1),
                seats: SeatCount(4),
                oar_configuation: OarsPerSeat(1),
            } => Some(BoatType::FourPlus),
            BoatAttributes {
                has_cox: HasCox(1),
                seats: SeatCount(8),
                oar_configuation: OarsPerSeat(1),
            } => Some(BoatType::Eight),
            _ => None,
        }
    }
}

impl std::fmt::Display for BoatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BoatType::Single => "Single",
            BoatType::Double => "Double",
            BoatType::DoublePlus => "Double+",
            BoatType::Quad => "Quad",
            BoatType::QuadPlus => "Quad+",
            BoatType::Octo => "Octo",
            BoatType::OctoPlus => "Octo+",
            BoatType::Pair => "Pair",
            BoatType::PairPlus => "Pair+",
            BoatType::Four => "Four",
            BoatType::FourPlus => "Four+",
            BoatType::Eight => "Eight",
        };
        f.write_str(s)
    }
}

pub struct BoatAttributes {
    pub has_cox: HasCox,
    pub seats: SeatCount,
    pub oar_configuation: OarsPerSeat,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "ssr", derive(diesel_derive_enum::DbEnum))]
#[cfg_attr(feature = "ssr", DbValueStyle = "verbatim")]
pub enum WeightClass {
    Light,
    Medium,
    Heavy,
    Tubby,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    diesel_derive_newtype::DieselNewType,
)]
pub struct HasCox(i32);

impl HasCox {
    pub fn new(has_cox: bool) -> Self {
        if has_cox {
            Self(1)
        } else {
            Self(0)
        }
    }
    pub fn as_bool(&self) -> bool {
        self.0 != 0
    }
    pub fn as_value(&self) -> i32 {
        self.0
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    diesel_derive_newtype::DieselNewType,
)]
pub struct SeatCount(i32);

impl SeatCount {
    pub fn new(count: i32) -> Option<Self> {
        match count {
            1 | 2 | 4 | 8 => Some(Self(count)),
            _ => None,
        }
    }
    pub fn count(&self) -> i32 {
        self.0
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    diesel_derive_newtype::DieselNewType,
)]
pub struct OarsPerSeat(i32);

impl OarsPerSeat {
    pub fn new(count: i32) -> Option<Self> {
        match count {
            1 | 2 => Some(Self(count)),
            _ => None,
        }
    }
    pub fn from_oar_configuration(oar_config: OarConfiguration) -> Self {
        match oar_config {
            OarConfiguration::Sweep => Self(1),
            OarConfiguration::Scull => Self(2),
        }
    }
    pub fn count(&self) -> i32 {
        self.0
    }
    pub fn configuration(&self) -> OarConfiguration {
        match self.0 {
            1 => OarConfiguration::Sweep,
            2 => OarConfiguration::Scull,
            _ => panic!("Oars per seat must be either 1 or 2"),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum OarConfiguration {
    Sweep,
    Scull,
}
impl OarConfiguration {
    pub fn num_oars(&self) -> i32 {
        match self {
            OarConfiguration::Sweep => 1,
            OarConfiguration::Scull => 2,
        }
    }
}
