use std::collections::HashMap;
use yew::{Component, Callback, Context, html, Html, Properties};
use web_sys::FileList;
use wasm_bindgen_futures::spawn_local;
use gloo_file::{callbacks::FileReader, File};
use graphql_client::GraphQLQuery;
use serde_json::Value;
use log::debug;
use crate::error::{get_error, Error};
use crate::services::Requests;
use crate::types::{UUID, UploadFile};
use crate::gqls::make_query;
use crate::gqls::relate::{
    ConfirmUploadCompleted, confirm_upload_completed
};

#[derive(Default, Debug)]
pub struct UploadData {
    pub upload_url: String,
    pub file_data: Vec<u8>,
}

pub enum Msg {
    Loaded(UploadFile, Vec<u8>),
    ParseFiles,
    ResultUpload(Result<Option<String>, Error>),
    ReqUploadCompleted,
    GetUploadConfirm(String),
    Clear,
}

pub struct StorageUpload {
    readers: HashMap<String, FileReader>,
    confirm_upload: Vec<UUID>,
    upload_file: Requests,
    upload_result: Callback<Result<Option<String>, Error>>,
    noconfirm_files: usize,
}

#[derive(Properties, Clone, Debug, PartialEq)]
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
        let props = ctx.props().clone();

        match msg {
            Msg::Loaded(file_info, data) => {
                debug!("request: {:?}", file_info);
                // self.files.push();
                self.upload_file.put_file(
                    file_info.upload_url.as_str(),
                    data.clone(),
                    self.upload_result.clone()
                );
                self.readers.remove(&file_info.filename);
                self.confirm_upload.push(file_info.file_uuid.clone());
            },
            Msg::ParseFiles => {
                for (file_info, file) in props.data_upload.into_iter() {
                    let l_link = ctx.link().clone();
                    let filename = file_info.filename.clone();
                    let task = {
                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            l_link.send_message(Msg::Loaded(
                                file_info.clone(),
                                // file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.noconfirm_files += 1;
                    self.readers.insert(filename, task);
                }
                debug!("Кол-во файлов для загрузки: {:?}", self.noconfirm_files);
            },
            Msg::ResultUpload(Ok(_)) => {
                debug!("next: {:?}", self.noconfirm_files);
                self.noconfirm_files -= 1;
                if self.noconfirm_files == 0 {
                    // self.get_result_up_file = true;
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
                        ctx.props().callback_confirm.emit(Ok(result));
                        ctx.link().send_message(Msg::Clear);
                    },
                    true => ctx.props().callback_confirm.emit(Err(get_error(&data))),
                }
            },
            Msg::Clear => {
                self.readers = HashMap::default();
                self.confirm_upload = Vec::default();
                self.upload_file = Requests::new();
                self.upload_result = ctx.link().callback(Msg::ResultUpload);
                self.noconfirm_files = 0;
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.data_upload.first().map(|d| &d.0.file_uuid) ==
              ctx.props().data_upload.first().map(|d| &d.0.file_uuid) {
            false
        } else {
            debug!("Получение новых данных для загрузки...");
            ctx.link().send_message(Msg::ParseFiles);
            true
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}

pub fn storage_upload(
    // &self,
    info_data: Vec<UploadFile>,
    files: Vec<File>,
    callback_confirm: Callback<Result<usize, Error>>,
) -> Html {
    let mut data_upload: Vec<(UploadFile, File)> = Vec::new();
    for value in info_data.into_iter().rev().zip(files).map(|value| value).into_iter() {
        debug!("parse data for upload...{:?}", value.0.filename);
        data_upload.push(value);
    };

    html!{
        <StorageUpload
            {data_upload}
            {callback_confirm}
        />
    }
}

/// Собирает данные о файлах из input в вектор
pub fn prepare_files(
    file_list: &Option<FileList>,
    files: &mut Vec<File>,
) {
    if let Some(fl) = file_list {
        let mut get_i: u32 = 0;
        while let Some(file) = fl.get(get_i).map(|f| File::from(f)) {
            files.push(file);
            get_i +=1;
        }
    }
}
