use std::collections::HashMap;
use base64::encode;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use yew::{Component, Callback, Context, html, Html, Properties};
use yew::html::TargetCast;
use serde_json::Value;
use log::debug;
use crate::error::{get_error, Error};
use crate::services::Requests;
use crate::types::{UUID, UploadFile};
use crate::gqls::make_query;
use crate::gqls::relate::{ConfirmUploadCompleted, confirm_upload_completed};


#[derive(Default, Debug)]
pub struct UploadData {
    pub upload_url: String,
    pub file_data: Vec<u8>,
}

struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(UploadFile, Vec<u8>),
    ParseFiles,
    ResultUpload(Result<(), Error>),
    ReqUploadCompleted,
    GetUploadConfirm(String),
}

pub struct StorageUpload {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
    confirm_upload: Vec<UUID>,
    upload_file: Requests,
    upload_result: Callback<Result<Option<String>, Error>>,
    noconfirm_files: usize,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub data_upload: Vec<(UploadFile, File)>,
    pub callback_confirm: Callback<Result<usize, Error>>,
}

impl Component for StorageUpload {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
            confirm_upload: Vec::default(),
            upload_file: Requests::new(),
            upload_result: ctx.link().callback(Msg::ResultUpload),
            noconfirm_files: 0,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::ParseFiles);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::Loaded(file_info, data) => {
                debug!("request: {:?}", file_info);
                self.files.push(
                    self.upload_file.put_file(
                        file_info.upload_url.as_str(),
                        &data,
                        self.upload_result.clone()
                    )
                );
                self.readers.remove(&file_info.filename);
                self.confirm_upload.push(file_info.file_uuid.clone());
            },
            Msg::ParseFiles => {
                for (file_info, file) in ctx.props().data_upload.iter() {
                    // spawn_local(async move {
                    //
                    // });
                    let task = {
                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_info,
                                // file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.noconfirm_files += 1;
                    self.readers.insert(file_info.filename, task);
                }
                debug!("Кол-во файлов для загрузки: {:?}", self.noconfirm_files);
            },
            Msg::ResultUpload(Ok(())) => {
                debug!("next: {:?}", self.noconfirm_files);
                self.noconfirm_files -= 1;
                if self.noconfirm_files == 0 {
                    self.get_result_up_file = true;
                    debug!("finish: {:?}", self.confirm_upload.len());
                    link.send_message(Msg::ReqUploadCompleted);
                }
            },
            Msg::ResultUpload(Err(err)) => {
                debug!("ResultUpload: {:?}", err);
                // todo!(тут очистка данных)
            },
            Msg::ReqUploadCompleted => {
                let file_uuids = self.confirm_upload.clone();
                spawn_local(async move {
                    let res =
                        make_query(
                            ConfirmUploadCompleted::build_query(
                                confirm_upload_completed::Variables { file_uuids }
                            )
                        ).await.unwrap();
                    // debug!("ConfirmUploadCompleted: {:?}", res);
                    link.send_message(Msg::GetUploadConfirm(res));
                });
            },
            Msg::GetUploadConfirm(res) => {
                let data: Value = serde_json::from_str(res.as_str()).unwrap();
                let res_value = data.as_object().unwrap().get("data").unwrap();

                match res_value.is_null() {
                    false => {
                        let result: usize =
                            serde_json::from_value(res_value.get("uploadCompleted").unwrap().clone()).unwrap();
                        debug!("uploadCompleted: {:?}", result);

                        ctx.props().callback_confirm(Ok(result));
                    },
                    true => ctx.props().callback_confirm(Err(get_error(&data))),
                }
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}
