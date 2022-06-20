use crate::services::{set_list_view, get_list_view};

#[derive(PartialEq, Eq)]
pub enum ListState {
    List,
    Box,
}

impl ListState {
    pub fn set_to_storage(list_view: &ListState) {
        match list_view {
            ListState::List => set_list_view(Some(String::from("List"))),
            ListState::Box => set_list_view(Some(String::from("Box"))),
        }
    }

    pub fn get_from_storage() -> Self {
        match get_list_view() {
            Some(ref t) if t == "List" => ListState::List,
            _ => ListState::Box,
        }
    }
}
