use yew::callback::Callback;
use super::Requests;
use crate::error::Error;
use crate::types::*;

/// Apis for tags
#[derive(Default, Debug)]
pub struct Tags {
    requests: Requests,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Get all tags
    pub fn get_all(&mut self, callback: Callback<Result<TagListInfo, Error>>) -> () {
        self.requests.get::<TagListInfo>("/tags", callback)
    }
}
