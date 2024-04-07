use super::*;

#[server(GetBoats2)]
pub(crate) async fn search_boats(
    filter: BoatFilter3,
    search_name: Option<String>,
) -> Result<Vec<Boat>, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    tracing::info!(?search_name, ?filter);
    conn.interact(|conn| Boat::get_boats3(conn, filter, search_name).map_err(ServerFnError::from))
        .await?
}

#[server(SubmitBoats)]
pub(crate) async fn submit_boats(
    boat_ids: Vec<BoatId>,
    session_type: UseScenario,
    recorded_at: NaiveDateTime,
) -> Result<BatchId, ServerFnError> {
    // let state: crate::ui::state::AppState = extract().await.expect("to get state aoeu");
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    let new_batch = crate::db::use_event_batch::NewBatchArgs {
        boat_ids,
        batch: crate::db::use_event_batch::NewBatch {
            use_scenario: session_type,
            recorded_at,
        },
    };
    conn.interact(|conn| {
        crate::db::use_event_batch::UseEventBatch::create_batch(conn, new_batch)
            .map_err(ServerFnError::from)
    })
    .await?
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ExistingBatch {
    pub(super) batch: Option<UseEventBatch>,
    pub(super) batch_entries: Vec<(UseEvent, Boat)>,
}
#[server(GetExistingBatch2)]
pub(crate) async fn get_existing_batch(batch_id: BatchId) -> Result<ExistingBatch, ServerFnError> {
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    let batch_entries = conn.interact(move |conn| {
        crate::db::use_event_batch::UseEventBatch::get_events_and_boats_for_batch(conn, batch_id)
            .map_err(ServerFnError::new)
    });
    let batch = conn.interact(move |conn| {
        crate::db::use_event_batch::UseEventBatch::get_batch(conn, batch_id)
            .map_err(ServerFnError::new)
    });
    let (batch_entries, batch) = futures::future::try_join(batch_entries, batch).await?;
    Ok(ExistingBatch {
        batch: batch?,
        batch_entries: batch_entries?,
    })
}

#[server(ReplaceBatch)]
pub(crate) async fn replace_batch(
    batch_id: BatchId,
    boat_ids: Vec<BoatId>,
    use_type: Option<UseScenario>,
) -> Result<(), ServerFnError> {
    let state = crate::ui::state::AppState::singleton();
    let conn = state.pool().get().await?;
    conn.interact(move |conn| {
        // currently don't overwrite the recorded at field, because we don't support customizing it in the first place
        crate::db::use_event_batch::UseEventBatch::replace_batch_uses(
            conn, batch_id, boat_ids, use_type, None,
        )
        .map_err(ServerFnError::from)
        .map(|_| ())
    })
    .await?
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum BoatListMsg {
    /// Fetch the boats according to the search criteria.
    Fetch,
    /// Filter the search to only boats with sweep or sculling oar configurations.
    SetFilterOarConfig(Option<OarConfiguration>),
    /// Filter the search to only boats with or without a coxswain.
    SetFilterCoxed(Option<HasCox>),
    /// Filter the search to only boats with this number of seats
    SetFilterNumSeats(Option<SeatCount>),
    /// Set the search string used to filter boats in the right-hand-pane.
    SetSearch(String),
    /// Remove an item from the right-hand-pane, adding it to the left-hand-pane.
    /// This item will be saved as part of the batch.
    AddToBatch(BoatId),
    /// Remove an item from the left-hand-pane.
    /// This should cause it to show back up in the other pane as it is no longer filtered.
    RemoveFromBatch(BoatId),
    /// Only used when in edit mode.
    /// This should, in a transaction, delete the use events tied to the batch,
    /// and re-create new ones that reflect the boat ids passed in.
    /// The created at time should also be set to the value provided.
    SaveChanges {
        batch_id: BatchId,
        boat_ids: Vec<BoatId>,
    },
    /// Submits the selected boats, saving use events for each one.
    Submit,
}

pub(super) async fn boat_list_service(
    mut rx: UnboundedReceiver<BoatListMsg>,
    mut searched_boats: Signal<Vec<Boat>>,
    mut selected_boats: Signal<Vec<Boat>>,
    mut filter: Signal<BoatFilter3>,
    mut search_name: Signal<Option<String>>,
    session_type: Signal<UseScenario>,
    mut created_at_time: Signal<String>,
    toasts: Coroutine<ToastMsgMsg>,
) {
    use futures::stream::StreamExt;

    let search = {
        to_owned![filter, search_name];
        move || async move {
            let _ = search_boats(filter.read().to_owned(), search_name.read().to_owned())
                .await
                .map(|boats| {
                    let exclude: HashSet<BoatId> =
                        selected_boats.read().iter().map(|x| x.id).collect();
                    boats
                        .into_iter()
                        .filter(|x| !exclude.contains(&x.id))
                        .collect()
                })
                .map(|x| searched_boats.set(x));
        }
    };

    while let Some(msg) = rx.next().await {
        match msg {
            BoatListMsg::Fetch => {
                tracing::info!("fetching");
                search().await
            }
            BoatListMsg::SaveChanges { batch_id, boat_ids } => {
                let session_type = *session_type.read();
                tracing::info!(?batch_id, ?boat_ids, "Overwriting old batch with new data");
                match replace_batch(batch_id, boat_ids, Some(session_type)).await {
                    Ok(_) => {
                        toasts.send(ToastData::info(format!("Edited batch {batch_id}")).into())
                    }
                    Err(error) => toasts.send(ToastData::error(error).into()),
                }
            }
            BoatListMsg::SetSearch(search_str) => {
                tracing::info!(%search_str, "setting search");
                if search_str.is_empty() {
                    search_name.set(None);
                } else {
                    search_name.set(Some(search_str));
                }
                search().await
            }
            BoatListMsg::SetFilterOarConfig(oars_config) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 {
                        _x: current._x,
                        num_seats: current.num_seats,
                        coxed: current.coxed,
                        oars_config,
                    }
                });
                search().await;
            }
            BoatListMsg::SetFilterCoxed(coxed) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 {
                        _x: current._x,
                        num_seats: current.num_seats,
                        coxed,
                        oars_config: current.oars_config,
                    }
                });
                search().await;
            }
            BoatListMsg::SetFilterNumSeats(num_seats) => {
                filter.set({
                    let current = filter.read();
                    BoatFilter3 {
                        _x: current._x,
                        num_seats,
                        coxed: current.coxed,
                        oars_config: current.oars_config,
                    }
                });
                search().await;
            }
            BoatListMsg::AddToBatch(id) => {
                tracing::info!(?id, "adding to batch");
                let mut boat_to_add = None;
                let new_searched_boats = {
                    searched_boats
                        .read()
                        .iter()
                        .cloned()
                        .filter_map(|boat| {
                            if boat.id == id {
                                boat_to_add = Some(boat);
                                None
                            } else {
                                Some(boat)
                            }
                        })
                        .collect()
                };
                searched_boats.set(new_searched_boats);
                if let Some(boat) = boat_to_add {
                    selected_boats.write().push(boat);
                }
            }
            BoatListMsg::RemoveFromBatch(id) => {
                let mut boat_to_remove = None;
                // O(n), retain the items that aren't the one at the id
                let new_selected_boats = {
                    selected_boats
                        .read()
                        .iter()
                        .cloned()
                        .filter_map(|boat| {
                            if boat.id == id {
                                boat_to_remove = Some(boat);
                                None
                            } else {
                                Some(boat)
                            }
                        })
                        .collect()
                };
                selected_boats.set(new_selected_boats);
                search().await;
            }
            BoatListMsg::Submit => {
                let ids: Vec<BoatId> = selected_boats.read().iter().map(|b| b.id).collect();
                let session_type = *session_type.read();
                let recorded_at = created_at_time();
                tracing::trace!(recorded= ?recorded_at.trim());

                // let date_result = DateTime::<FixedOffset>::parse_from_str(recorded_at.trim(), crate::ui::util::time::MINUTE_RESOLUTION_FMT);
                let date_result =
                    crate::ui::util::time::parse_str_as_naive_to_utc(recorded_at.trim());
                match date_result {
                    Ok(recorded_at) => {
                        // This recorded at should be input + 4h
                        tracing::info!(?recorded_at, input = created_at_time());
                        if !ids.is_empty() {
                            match submit_boats(ids, session_type, recorded_at).await {
                                Ok(id) => {
                                    tracing::info!(%id, "Created batch");
                                    searched_boats.set(Vec::new());
                                    selected_boats.set(Vec::new());
                                    search_name.set(None);
                                    created_at_time.set(crate::ui::util::time::render_local(
                                        chrono::Utc::now().naive_utc(),
                                    ));
                                    toasts.send(ToastData::success("Submitted boats").into());
                                    // refresh the search page
                                    search().await
                                }
                                Err(error) => {
                                    tracing::error!(?error, "Could not submit batch");
                                    toasts.send(
                                        ToastData::warn("Could not submit batch".to_string())
                                            .into(),
                                    );
                                }
                            }
                        }
                    }
                    Err(error) => {
                        toasts.send(ToastData::error(error).into());
                    }
                }
            }
        }
    }
}
