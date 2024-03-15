use std::ops::Deref;

use dioxus_fullstack::prelude::ServerFnError;

pub type LoadableResult<T> = Loadable<Result<T, ServerFnError>>;
pub type LoadableRefResult<'a, T> = Loadable<&'a Result<T, ServerFnError>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Loadable<T> {
    #[default]
    Loading,
    Loaded(T)
}

impl <T> From<T> for Loadable<T> {
    fn from(value: T) -> Self {
        Self::Loaded(value)
    }
}

impl <T> Loadable<T> {
    pub fn from_option(option: Option<T>) -> Self {
        match option {
            Some(x) => Self::Loaded(x),
            None => Self::Loading
        }
    }

    pub fn set_loading(&mut self) {
        *self = Self::Loading
    }

    pub fn as_ref(&self) -> Loadable<&T> {
        match self {
            Loadable::Loading => Loadable::Loading,
            Loadable::Loaded(x) => Loadable::Loaded(x),
        }

    }
}
impl <T: Deref> Loadable<T> {
    pub fn as_deref(&self) -> Loadable<&<T as Deref>::Target>{
        match self {
            Loadable::Loading => Loadable::Loading,
            Loadable::Loaded(x) => Loadable::Loaded(x.deref()),
        }
    }
    
}

