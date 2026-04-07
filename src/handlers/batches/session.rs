use axum_extra::extract::cookie::{Cookie, SameSite, SignedCookieJar};
use crate::db::boat::types::BoatId;

const SELECTED_BOATS_COOKIE: &str = "selected_boats";

/// The persisted state for boat selection during batch creation.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SelectedBoats {
    pub boat_ids: Vec<BoatId>,
}

impl SelectedBoats {
    pub fn add(&mut self, id: BoatId) {
        if !self.boat_ids.contains(&id) {
            self.boat_ids.push(id);
        }
    }

    pub fn remove(&mut self, id: BoatId) {
        self.boat_ids.retain(|existing| *existing != id);
    }
}

/// Reads the selected boats from the signed cookie.
/// Returns the default (empty) if the cookie is absent or malformed.
pub fn read_selected_boats(jar: &SignedCookieJar) -> SelectedBoats {
    jar.get(SELECTED_BOATS_COOKIE)
        .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
        .unwrap_or_default()
}

/// Writes the selected boats into the signed cookie.
pub fn write_selected_boats(jar: SignedCookieJar, selected: &SelectedBoats) -> SignedCookieJar {
    let json = serde_json::to_string(selected)
        .expect("serializing SelectedBoats should not fail");

    let cookie = Cookie::build((SELECTED_BOATS_COOKIE, json))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(false)
        .build();

    jar.add(cookie)
}

/// Removes the selected boats cookie.
pub fn clear_selected_boats(jar: SignedCookieJar) -> SignedCookieJar {
    jar.remove(Cookie::from(SELECTED_BOATS_COOKIE))
}
