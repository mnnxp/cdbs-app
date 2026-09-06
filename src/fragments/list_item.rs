use crate::services::{set_list_view, get_list_view};

#[derive(PartialEq, Eq)]
pub(crate) enum ListState {
    List,
    Box,
}

impl ListState {
    pub(crate) fn get_container_class(&self) -> &'static str {
        match self {
            ListState::Box => "flex-box",
            ListState::List => "",
        }
    }

    pub(crate) fn get_icon_class(&self) -> &'static str {
        match self {
            ListState::Box => "fas fa-bars",
            ListState::List => "fas fa-th-large",
        }
    }

    pub(crate) fn set_to_storage(list_view: &ListState) {
        match list_view {
            ListState::List => set_list_view(Some(String::from("List"))),
            ListState::Box => set_list_view(Some(String::from("Box"))),
        }
    }

    pub(crate) fn get_from_storage() -> Self {
        match get_list_view() {
            Some(ref t) if t == "Box" => ListState::Box,
            _ => ListState::List,
        }
    }
}
