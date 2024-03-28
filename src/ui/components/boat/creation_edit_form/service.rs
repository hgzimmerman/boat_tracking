use crate::{
    db::boat::{
        types::{BoatId, BoatType, WeightClass},
        Boat, NewBoat,
    }, ui::components::toast::{MsgType, ToastData, ToastMsgMsg}
};
use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[server(CreateBoats)]
pub(super) async fn create_boat(
    name: String,
    weight: WeightClass,
    ty: BoatType,
    acquired_at: Option<NaiveDate>,
    manufactured_at: Option<NaiveDate>,
) -> Result<Boat, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    let boat = NewBoat::new(name, weight, ty, acquired_at, manufactured_at);

    conn.interact(|conn| Boat::new_boat(conn, boat).map_err(ServerFnError::from))
        .await?
}

#[server(UpdateBoats)]
pub(super) async fn update_boat(
    name: String,
    weight: WeightClass,
    ty: BoatType,
    acquired_at: Option<NaiveDate>,
    manufactured_at: Option<NaiveDate>,
    relinquished_at: Option<NaiveDate>,
) -> Result<Boat, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    let boat = NewBoat::new(name, weight, ty, acquired_at, manufactured_at);

    conn.interact(|conn| Boat::new_boat(conn, boat).map_err(ServerFnError::from))
        .await?
}

#[server(GetBoat)]
pub(super) async fn get_boat(
    boat_id: BoatId
) -> Result<Boat, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let conn_string = "db.sql";
    let state = crate::ui::state::AppState::new(conn_string);
    let conn = state.pool().get().await?;

    conn.interact(move |conn| Boat::get_boat(conn, boat_id).map_err(ServerFnError::from))
        .await?
}

pub(super) enum CreateBoatMsg {
    Create,
    Update(BoatId)
}

struct UpdateBoatArgs {
    name: String,
    weight: WeightClass,
    ty: BoatType,
    acquired_at: Option<NaiveDate>,
    manufactured_at: Option<NaiveDate>,
    relinquished_at: Option<NaiveDate>,
}
impl UpdateBoatArgs {
    fn new(
        name: Signal<String>,
        weight: Signal<Option<WeightClass>>,
        ty: Signal<Option<BoatType>>,
        acquired_at: Signal<String>,
        manufactured_at: Signal<String>,
        relinquished_at: Option<Signal<String>>
    ) -> Result<Self, BoatArgsError> {
        let name: String = if name.read().is_empty() {
            return Err(BoatArgsError::MissingName);
        } else {
            name.read().clone()
        };
        let weight: WeightClass = if let Some(weight) = *weight.read() {
            weight
        } else {
            return Err(BoatArgsError::MissingWeight);
        };
        let ty: BoatType = if let Some(ty) = *ty.read() {
            ty
        } else {
            return Err(BoatArgsError::MissingBoatType);
        };
        let acquired_at = if acquired_at.read().is_empty() {
            None
        } else {
            tracing::info!(acquired = ?acquired_at.read());
            chrono::NaiveDate::parse_from_str(&&acquired_at.read(), "%Y-%m-%d")
                .map_err(BoatArgsError::InvalidAcquiredAt)
                .map(Some)?
        };
        let manufactured_at = if manufactured_at.read().is_empty() {
            None
        } else {
            tracing::info!(manufactured = ?manufactured_at.read());
            chrono::NaiveDate::parse_from_str(&&manufactured_at.read(), "%Y-%m-%d")
                .map_err(BoatArgsError::InvalidManufactureddAt)
                .map(Some)?
        };
        let relinquished_at: Option<NaiveDate> = relinquished_at.map(|relinquished_at| {
            if relinquished_at.read().is_empty() {
                Ok(None)
            } else {
                tracing::info!(relinquished = ?relinquished_at.read());
                chrono::NaiveDate::parse_from_str(&&relinquished_at.read(), "%Y-%m-%d")
                    .map_err(BoatArgsError::InvalidSoldAt)
                    .map(Some)
            }
        }).transpose()?.flatten();
        Ok(UpdateBoatArgs {
            name,
            weight,
            ty,
            acquired_at,
            manufactured_at,
            relinquished_at
        })
    }
}

struct CreateBoatArgs {
    name: String,
    weight: WeightClass,
    ty: BoatType,
    acquired_at: Option<NaiveDate>,
    manufactured_at: Option<NaiveDate>,
}
#[derive(Debug, Clone, thiserror::Error)]
enum BoatArgsError {
    #[error("Missing Name")]
    MissingName,
    #[error("Missing Weight")]
    MissingWeight,
    #[error("Missing Boat Type")]
    MissingBoatType,
    #[error("Could not parse Acquired date.")]
    InvalidAcquiredAt(chrono::ParseError),
    #[error("Could not parse Manufacturing date.")]
    InvalidManufactureddAt(chrono::ParseError),
    #[error("Could not parse Sold at date.")]
    InvalidSoldAt(chrono::ParseError),
}
impl CreateBoatArgs {
    fn new(
        name: Signal<String>,
        weight: Signal<Option<WeightClass>>,
        ty: Signal<Option<BoatType>>,
        acquired_at: Signal<String>,
        manufactured_at: Signal<String>,
    ) -> Result<Self, BoatArgsError> {
        let name: String = if name.read().is_empty() {
            return Err(BoatArgsError::MissingName);
        } else {
            name.read().clone()
        };
        let weight: WeightClass = if let Some(weight) = *weight.read() {
            weight
        } else {
            return Err(BoatArgsError::MissingWeight);
        };
        let ty: BoatType = if let Some(ty) = *ty.read() {
            ty
        } else {
            return Err(BoatArgsError::MissingBoatType);
        };
        let acquired_at = if acquired_at.read().is_empty() {
            None
        } else {
            tracing::info!(acquired = ?acquired_at.read());
            chrono::NaiveDate::parse_from_str(&&acquired_at.read(), "%Y-%m-%d")
                .map_err(BoatArgsError::InvalidAcquiredAt)
                .map(Some)?
        };
        let manufactured_at = if manufactured_at.read().is_empty() {
            None
        } else {
            tracing::info!(manufactured = ?manufactured_at.read());
            chrono::NaiveDate::parse_from_str(&&manufactured_at.read(), "%Y-%m-%d")
                .map_err(BoatArgsError::InvalidManufactureddAt)
                .map(Some)?
        };
        Ok(CreateBoatArgs {
            name,
            weight,
            ty,
            acquired_at,
            manufactured_at,
        })
    }
}

pub(super) async fn create_boat_service(
    mut rx: UnboundedReceiver<CreateBoatMsg>,
    mut name: Signal<String>,
    mut weight: Signal<Option<WeightClass>>,
    mut ty: Signal<Option<BoatType>>,
    mut acquired_at: Signal<String>,
    mut manufactured_at: Signal<String>,
    relinquished_at: Option<Signal<String>>,
    toasts: Coroutine<ToastMsgMsg>,
) {
    use futures::stream::StreamExt;

    while let Some(msg) = rx.next().await {
        match msg {
            CreateBoatMsg::Update(id) => {
                match UpdateBoatArgs::new(name, weight, ty, acquired_at, manufactured_at, relinquished_at) {
                    Ok(args) => {
                        match update_boat(
                            args.name,
                            args.weight,
                            args.ty,
                            args.acquired_at,
                            args.manufactured_at,
                            args.relinquished_at,
                        )
                        .await {
                            Ok(boat) => {
                                name.set(String::new());
                                weight.set(None);
                                ty.set(None);
                                acquired_at.set(String::new());
                                manufactured_at.set(String::new());
                                if let Some(mut relinquished_at) = relinquished_at {
                                    relinquished_at.set(String::new());
                                }

                                toasts.send(ToastData::info(format!("Updated boat '{}' with id '{id}'.", boat.name)).into());
                            }
                            Err(error) => {
                                tracing::warn!(?error, "Could not send request");
                                toasts.send(ToastData::error(error).into())
                            }
                        }
                    },
                    Err(error) => {
                        tracing::warn!(?error, "failed validation");
                        toasts.send(ToastData::error(error).into());
                    }
                }
            }
            CreateBoatMsg::Create => {
                match CreateBoatArgs::new(name, weight, ty, acquired_at, manufactured_at) {
                    Ok(args) => {
                        match create_boat(
                            args.name,
                            args.weight,
                            args.ty,
                            args.acquired_at,
                            args.manufactured_at,
                        )
                        .await
                        {
                            Ok(_boat) => {
                                name.set(String::new());
                                weight.set(None);
                                ty.set(None);
                                acquired_at.set(String::new());
                                manufactured_at.set(String::new());

                                toasts.send(ToastMsgMsg::Add(
                                    ToastData {
                                        msg: "Created new boat".to_string(),
                                        ty: MsgType::Normal,
                                    },
                                    std::time::Duration::from_secs(5),
                                ))
                            }
                            Err(error) => {
                                tracing::warn!(?error, "Could not send request");

                                toasts.send(ToastMsgMsg::Add(
                                    ToastData {
                                        msg: "Could not send requests".to_string(),
                                        ty: MsgType::Error,
                                    },
                                    std::time::Duration::from_secs(2),
                                ))
                            }
                        }
                    }
                    Err(error) => {
                        tracing::warn!(?error, "failed validation");
                        toasts.send(ToastMsgMsg::Add(
                            ToastData {
                                msg: error.to_string(),
                                ty: MsgType::Warn,
                            },
                            std::time::Duration::from_secs(2),
                        ));
                    }
                }
            }
        }
    }
}