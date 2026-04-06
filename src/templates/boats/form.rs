use maud::{html, Markup};
use crate::db::boat::{Boat, types::{BoatId, BoatType, WeightClass}};

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
        div class="flex-grow flex flex-col items-center bg-gray-50 dark:bg-gray-600 p-8" {
            (boat_form(mode, data, errors))
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

    // Compute CSS classes for error states
    let name_border_class = if errors.name.is_some() { "border-red-500" } else { "border-gray-300" };
    let name_input_class = format!("w-full bg-white border {name_border_class} rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-600 dark:border-gray-500 dark:text-white");

    let acquired_border_class = if errors.acquired_at.is_some() { "border-red-500" } else { "border-gray-300" };
    let acquired_input_class = format!("w-full bg-white border {acquired_border_class} rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-600 dark:border-gray-500 dark:text-white");

    let manufactured_border_class = if errors.manufactured_at.is_some() { "border-red-500" } else { "border-gray-300" };
    let manufactured_input_class = format!("w-full bg-white border {manufactured_border_class} rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-600 dark:border-gray-500 dark:text-white");

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

    html! {
        form
            action=(action)
            method=(method)
            class="bg-white shadow-md rounded-lg px-8 pt-6 pb-8 w-full max-w-2xl dark:bg-gray-700"
            hx-post=(action)
            hx-target="#content"
            {
            h2 class="mb-6 text-3xl font-extrabold text-gray-900 dark:text-white" {
                (title)
            }

            @if errors.has_errors() {
                div class="mb-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded" {
                    p class="font-bold" { "Please fix the following errors:" }
                    ul class="list-disc list-inside mt-2" {
                        @if let Some(err) = &errors.name {
                            li { (err) }
                        }
                        @if let Some(err) = &errors.weight_class {
                            li { (err) }
                        }
                        @if let Some(err) = &errors.boat_type {
                            li { (err) }
                        }
                        @if let Some(err) = &errors.acquired_at {
                            li { (err) }
                        }
                        @if let Some(err) = &errors.manufactured_at {
                            li { (err) }
                        }
                    }
                }
            }

            // Name field
            div class="mb-4" {
                label for="name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" {
                    "Boat Name"
                    span class="text-red-500" { "*" }
                }
                input
                    type="text"
                    id="name"
                    name="name"
                    value=(data.name)
                    required
                    class=(name_input_class);
                @if let Some(err) = &errors.name {
                    p class="mt-1 text-sm text-red-600" { (err) }
                }
            }

            // Weight Class dropdown
            (crate::templates::components::simple_select("weight_class", "Weight Class *", &WEIGHT_CLASSES, weight_class_selected))
            @if let Some(err) = &errors.weight_class {
                p class="mt-1 text-sm text-red-600" { (err) }
            }

            // Boat Type dropdown
            (crate::templates::components::simple_select("boat_type", "Boat Type", &BOAT_TYPES, boat_type_selected))
            @if let Some(err) = &errors.boat_type {
                p class="mt-1 text-sm text-red-600" { (err) }
            }

            // Acquired At date field
            div class="mb-4" {
                label for="acquired_at" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" {
                    "Acquired Date"
                }
                input
                    type="date"
                    id="acquired_at"
                    name="acquired_at"
                    value=(data.acquired_at)
                    class=(acquired_input_class);
                @if let Some(err) = &errors.acquired_at {
                    p class="mt-1 text-sm text-red-600" { (err) }
                }
            }

            // Manufactured At date field
            div class="mb-4" {
                label for="manufactured_at" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" {
                    "Manufactured Date"
                }
                input
                    type="date"
                    id="manufactured_at"
                    name="manufactured_at"
                    value=(data.manufactured_at)
                    class=(manufactured_input_class);
                @if let Some(err) = &errors.manufactured_at {
                    p class="mt-1 text-sm text-red-600" { (err) }
                }
            }

            // Relinquished At date field (only for edit mode)
            @if let BoatFormMode::Edit(_) = mode {
                div class="mb-4" {
                    label for="relinquished_at" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" {
                        "Relinquished Date"
                    }
                    input
                        type="date"
                        id="relinquished_at"
                        name="relinquished_at"
                        value=(data.relinquished_at.as_deref().unwrap_or(""))
                        class="w-full bg-white border border-gray-300 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-600 dark:border-gray-500 dark:text-white";
                }
            }

            // Submit and Cancel buttons
            div class="flex items-center justify-between mt-6" {
                button
                    type="submit"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-lg focus:outline-none focus:shadow-outline transition"
                    {
                    (submit_text)
                }
                a
                    href="/boats"
                    class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
                    {
                    "Cancel"
                }
            }
        }
    }
}
