use yew::callback::Callback;
use yew::services::fetch::FetchTask;
// use log::debug;

use super::Requests;
use crate::error::Error;

#[derive(Default, Debug)]
pub struct UploadData {
    pub filename: String,
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
        progress_callback: Callback<(Option<String>, f32)>,
    ) -> FetchTask {
        // debug!("File data: {:?}", upload_data.file_data);
        self.requests.put_f::<String>(
            upload_data,
            callback,
            progress_callback,
        )
    }
}
