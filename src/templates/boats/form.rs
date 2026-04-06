use maud::{html, Markup};
use crate::db::boat::{Boat, types::{BoatId, BoatType, WeightClass}};
use crate::templates::components::common::{form_card, form_label, form_error, input_class, BTN_PRIMARY};

/// Form mode - creating new boat or editing existing
#[derive(Debug, Clone, Copy)]
pub enum BoatFormMode {
    New,
    Edit(BoatId),
}

/// Validation errors for boat form
#[derive(Debug, Default)]
pub struct BoatFormErrors {
    pub name: Option<String>,
    pub weight_class: Option<String>,
    pub boat_type: Option<String>,
    pub acquired_at: Option<String>,
    pub manufactured_at: Option<String>,
}

impl BoatFormErrors {
    pub fn has_errors(&self) -> bool {
        self.name.is_some()
            || self.weight_class.is_some()
            || self.boat_type.is_some()
            || self.acquired_at.is_some()
            || self.manufactured_at.is_some()
    }

    fn error_list(&self) -> Vec<&str> {
        let mut errors = Vec::new();
        if let Some(e) = &self.name { errors.push(e.as_str()); }
        if let Some(e) = &self.weight_class { errors.push(e.as_str()); }
        if let Some(e) = &self.boat_type { errors.push(e.as_str()); }
        if let Some(e) = &self.acquired_at { errors.push(e.as_str()); }
        if let Some(e) = &self.manufactured_at { errors.push(e.as_str()); }
        errors
    }
}

/// Data for boat form (either from existing boat or empty for new)
#[derive(Debug, Clone)]
pub struct BoatFormData {
    pub name: String,
    pub weight_class: Option<WeightClass>,
    pub boat_type: Option<BoatType>,
    pub acquired_at: String,
    pub manufactured_at: String,
    pub relinquished_at: Option<String>,
}

impl BoatFormData {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            weight_class: None,
            boat_type: None,
            acquired_at: String::new(),
            manufactured_at: String::new(),
            relinquished_at: None,
        }
    }

    pub fn from_boat(boat: &Boat) -> Self {
        Self {
            name: boat.name.clone(),
            weight_class: Some(boat.weight_class),
            boat_type: boat.boat_type(),
            acquired_at: boat
                .acquired_at
                .map(|d| d.to_string())
                .unwrap_or_default(),
            manufactured_at: boat
                .manufactured_at
                .map(|d| d.to_string())
                .unwrap_or_default(),
            relinquished_at: boat.relinquished_at.map(|d| d.to_string()),
        }
    }
}

/// Weight class options for dropdown
const WEIGHT_CLASSES: [(&str, &str); 5] = [
    ("", "Select weight class..."),
    ("Light", "Light"),
    ("Medium", "Medium"),
    ("Heavy", "Heavy"),
    ("Tubby", "Tubby"),
];

/// Boat type options for dropdown
const BOAT_TYPES: [(&str, &str); 9] = [
    ("", "Select boat type..."),
    ("Single", "Single"),
    ("Double", "Double"),
    ("Quad", "Quad"),
    ("QuadPlus", "Quad+"),
    ("Pair", "Pair"),
    ("Four", "Four"),
    ("FourPlus", "Four+"),
    ("Eight", "Eight"),
];

/// Full page for boat form
pub fn boat_form_page(mode: BoatFormMode, data: BoatFormData, errors: BoatFormErrors) -> Markup {
    let title = match mode {
        BoatFormMode::New => "Add a New Boat",
        BoatFormMode::Edit(_) => "Edit Boat",
    };

    crate::templates::layout::page(title, boat_form_content(mode, data, errors))
}

/// Boat form content (without page wrapper)
pub fn boat_form_content(mode: BoatFormMode, data: BoatFormData, errors: BoatFormErrors) -> Markup {
    html! {
        div class="flex-grow flex flex-col bg-gray-50 dark:bg-slate-600" {
            // Include tabs for edit mode
            @if let BoatFormMode::Edit(boat_id) = mode {
                (super::detail::boat_tabs(boat_id.as_int(), "edit"))
            }
            div class="flex-grow flex flex-col items-center p-8" {
                (boat_form(mode, data, errors))
            }
        }
    }
}

/// Boat form component
pub fn boat_form(mode: BoatFormMode, data: BoatFormData, errors: BoatFormErrors) -> Markup {
    let (action, method) = match mode {
        BoatFormMode::New => ("/boats".to_string(), "post"),
        BoatFormMode::Edit(id) => (format!("/boats/{}", id.as_int()), "post"),
    };

    let title = match mode {
        BoatFormMode::New => "Add a New Boat".to_string(),
        BoatFormMode::Edit(_) => format!("Editing: {}", data.name),
    };

    let submit_text = match mode {
        BoatFormMode::New => "Create Boat",
        BoatFormMode::Edit(_) => "Update Boat",
    };

    // Compute dropdown selected values
    let weight_class_selected = data.weight_class.map(|wc| match wc {
        WeightClass::Light => "Light",
        WeightClass::Medium => "Medium",
        WeightClass::Heavy => "Heavy",
        WeightClass::Tubby => "Tubby",
    });

    let boat_type_selected = data.boat_type.map(|bt| match bt {
        BoatType::Single => "Single",
        BoatType::Double => "Double",
        BoatType::DoublePlus => "DoublePlus",
        BoatType::Quad => "Quad",
        BoatType::QuadPlus => "QuadPlus",
        BoatType::Pair => "Pair",
        BoatType::Four => "Four",
        BoatType::FourPlus => "FourPlus",
        BoatType::Eight => "Eight",
        BoatType::Octo => "Octo",
        BoatType::OctoPlus => "OctoPlus",
        BoatType::PairPlus => "PairPlus",
    });

    let errors_markup = if errors.has_errors() {
        let error_list = errors.error_list();
        html! {
            div class="mb-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded" {
                p class="font-bold" { "Please fix the following errors:" }
                ul class="list-disc list-inside mt-2" {
                    @for err in &error_list {
                        li { (err) }
                    }
                }
            }
        }
    } else {
        html! {}
    };

    let form_content = html! {
        form
            action=(action)
            method=(method)
            hx-post=(action)
            hx-target="#content"
            {
            // Name field
            div class="mb-4" {
                (form_label("name", "Boat Name", true))
                input
                    type="text"
                    id="name"
                    name="name"
                    value=(data.name)
                    required
                    class=(input_class(errors.name.is_some()));
                (form_error(errors.name.as_ref()))
            }

            // Weight Class dropdown
            (crate::templates::components::simple_select("weight_class", "Weight Class *", &WEIGHT_CLASSES, weight_class_selected))
            (form_error(errors.weight_class.as_ref()))

            // Boat Type dropdown
            (crate::templates::components::simple_select("boat_type", "Boat Type", &BOAT_TYPES, boat_type_selected))
            (form_error(errors.boat_type.as_ref()))

            // Acquired At date field
            div class="mb-4" {
                (form_label("acquired_at", "Acquired Date", false))
                input
                    type="date"
                    id="acquired_at"
                    name="acquired_at"
                    value=(data.acquired_at)
                    class=(input_class(errors.acquired_at.is_some()));
                (form_error(errors.acquired_at.as_ref()))
            }

            // Manufactured At date field
            div class="mb-4" {
                (form_label("manufactured_at", "Manufactured Date", false))
                input
                    type="date"
                    id="manufactured_at"
                    name="manufactured_at"
                    value=(data.manufactured_at)
                    class=(input_class(errors.manufactured_at.is_some()));
                (form_error(errors.manufactured_at.as_ref()))
            }

            // Relinquished At date field (only for edit mode)
            @if let BoatFormMode::Edit(_) = mode {
                div class="mb-4" {
                    (form_label("relinquished_at", "Relinquished Date", false))
                    input
                        type="date"
                        id="relinquished_at"
                        name="relinquished_at"
                        value=(data.relinquished_at.as_deref().unwrap_or(""))
                        class=(input_class(false));
                }
            }

            // Submit and Cancel buttons
            div class="flex items-center justify-between mt-6" {
                button type="submit" class=(BTN_PRIMARY) {
                    (submit_text)
                }
                a
                    href="/boats"
                    class="font-bold text-sm text-blue-500 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
                    {
                    "Cancel"
                }
            }
        }
    };

    form_card(&title, errors_markup, form_content)
}
