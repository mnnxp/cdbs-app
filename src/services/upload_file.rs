use yew::callback::Callback;
use web_sys::XmlHttpRequest;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use crate::error::Error;
use crate::types::ErrorInfo;
// use log::debug;

#[derive(Default, Debug)]
pub(crate) struct UploadData {
    pub(crate) filename: String,
    pub(crate) upload_url: String,
    pub(crate) file_data: Vec<u8>,
}

/// Put file data in storage
pub(crate) fn put_file(
    upload_data: UploadData,
    callback: Callback<Result<Option<String>, Error>>,
    progress_callback: Callback<(Option<String>, f32)>,
) {
    let xhr = XmlHttpRequest::new().unwrap();
    let filename = upload_data.filename.clone();

    xhr.open("PUT", &upload_data.upload_url).unwrap();

    let progress_cb = progress_callback.clone();
    let fname_for_progress = filename.clone();
    let on_progress = Closure::wrap(Box::new(move |e: web_sys::ProgressEvent| {
        if e.length_computable() {
            let percent = (e.loaded() as f64 / e.total() as f64) as f32;
            // debug!("Upload progress for {}: {:.2}%", fname_for_progress, percent * 100.0);
            progress_cb.emit((Some(fname_for_progress.clone()), percent));
        }
    }) as Box<dyn FnMut(_)>);
    xhr.upload().unwrap().set_onprogress(Some(on_progress.as_ref().unchecked_ref()));
    on_progress.forget();

    let cb_load = callback.clone();
    let xhr_c = xhr.clone();
    let on_load = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let status = xhr_c.status().unwrap();

        if status >= 200 && status < 300 {
            cb_load.emit(Ok(None));
        } else {
            match status {
                401 => cb_load.emit(Err(Error::Unauthorized)),
                403 => cb_load.emit(Err(Error::Forbidden)),
                404 => cb_load.emit(Err(Error::NotFound)),
                500 => cb_load.emit(Err(Error::InternalServerError)),
                422 => {
                    let text = xhr_c.response_text().unwrap_or_default().unwrap_or_default();
                    if let Ok(err_info) = serde_json::from_str::<ErrorInfo>(&text) {
                        cb_load.emit(Err(Error::UnprocessableEntity(err_info)));
                    } else {
                        cb_load.emit(Err(Error::DeserializeError));
                    }
                }
                _ => cb_load.emit(Err(Error::RequestError)),
            }
        }
    }) as Box<dyn FnMut(web_sys::Event)>);

    xhr.set_onload(Some(on_load.as_ref().unchecked_ref()));
    on_load.forget();
    let body = js_sys::Uint8Array::from(&upload_data.file_data[..]);
    xhr.send_with_opt_buffer_source(Some(&body)).unwrap();
}
