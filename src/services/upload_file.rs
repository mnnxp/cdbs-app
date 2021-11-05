use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

use graphql_client::GraphQLQuery;
use serde_json::Value;

use super::Requests;
use crate::error::{Error, get_error};
use crate::types::*;

#[derive(Default, Debug)]
pub struct UploadData {
    pub upload_url: String,
    pub file_data: Vec<u8>,
}

/// Apis for upload file
#[derive(Default, Debug)]
pub struct PutUploadFile {
    requests: Requests,
}

impl PutUploadFile {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Put file data in storage
    pub fn put_file(
        &mut self,
        upload_data: UploadData,
        callback: Callback<Result<Option<String>, Error>>,
    ) -> FetchTask {
        // ConsoleService::info(format!("File data: {:?}", &upload_data.file_data).as_ref());
        self.requests.put_f::<Vec<u8>, String>(
            upload_data.upload_url,
            upload_data.file_data,
            callback,
        )
    }
}
