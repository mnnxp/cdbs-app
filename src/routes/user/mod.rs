pub mod login;
pub mod register;
pub mod settings;
pub mod notification;
pub mod profile;

use yew::{html, Html};
use crate::services::content_adapter::ContentDisplay;
use crate::types::ShowUserShort;

impl ContentDisplay for ShowUserShort {
    /// Returns a username and firstname with lastname in p tag
    fn to_display(&self) -> Html {
        html!{
            <span title = {{format!("{} {}", self.firstname, self.lastname)}}>
                {format!("@{}", self.username)}
            </span>
        }
    }
}