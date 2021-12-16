use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
// use graphql_client::GraphQLQuery;
// use serde_json::Value;
// use wasm_bindgen_futures::spawn_local;

// use crate::error::{get_error, Error};
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
// use crate::gqls::make_query;
// use crate::types::{UUID, ShowFileInfo, DownloadFile};
use crate::types::{UUID, ShowFileInfo};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub file: ShowFileInfo,
}

pub struct FileOfFilesetItem {
    error: Option<Error>,
    props: Props,
    // link: ComponentLink<Self>,
    file_uuid: UUID,
    // open_full_info_file: bool,
    // get_result_delete: bool,
}

pub enum Msg {
    // RequestDownloadFile,
    // RequestDeleteFile,
    ResponseError(Error),
    // GetDownloadFileResult(String),
    // GetDeleteFileResult(String),
    // ClickFileInfo,
}

impl Component for FileOfFilesetItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            props,
            // link,
            file_uuid: String::new(),
            // open_full_info_file: false,
            // get_result_delete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();

        match msg {
            // Msg::RequestDownloadFile => {},
            // Msg::RequestDeleteFile => {},
            Msg::ResponseError(err) => self.error = Some(err),
            // Msg::GetDownloadFileResult(_res) => {},
            // Msg::GetDeleteFileResult(_res) => {},
            // Msg::ClickFileInfo => self.open_full_info_file = !self.open_full_info_file,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.file_uuid == props.file.uuid {
            false
        } else {
            self.file_uuid = props.file.uuid.clone();
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        // Filename:	test name file2.pdf
        // Content type:	application/text
        // Filesize:	0
        // Program:	AutoCAD
        // Upload by:	Lprotestor Dprotestor (@nuasw)
        // Upload at:	2021-12-13 12:41:08

        html! {<>
            <ListErrors error=self.error.clone()/>
            {self.show_full_info_file()}
        </>}
    }
}

impl FileOfFilesetItem {
    // fn show_download_btn(&self) -> Html {
    //     let onclick_download_btn = self.link
    //         .callback(|_| Msg::RequestDownloadFile);
    //
    //     match &self.props.show_download_btn {
    //         true => html!{
    //             <button class="button is-white is-small" onclick=onclick_download_btn >
    //               <span class="icon" >
    //                 <i class="fas fa-cloud-download-alt" aria-hidden="true"></i>
    //               </span>
    //             </button>
    //         },
    //         false => html!{},
    //     }
    // }
    
    // fn show_delete_btn(&self) -> Html {
    //     let onclick_delete_btn = self.link
    //         .callback(|_| Msg::RequestDeleteFile);
    //
    //     match &self.props.show_delete_btn {
    //         true => html!{
    //             <button class="button is-white is-small" onclick=onclick_delete_btn >
    //               <span class="icon" >
    //                 <i class="fa fa-trash" aria-hidden="true"></i>
    //               </span>
    //             </button>
    //         },
    //         false => html!{},
    //     }
    // }

    fn show_full_info_file(&self) -> Html {
        html!{<tr>
          // <td>{"Filename:"}</td>
          <td>{self.props.file.filename.clone()}</td>
          // <td>{"Content type:"}</td>
          <td>{self.props.file.content_type.clone()}</td>
          // <td>{"Filesize:"}</td>
          <td>{self.props.file.filesize.clone()}</td>
          // <td>{"Program:"}</td>
          <td>{self.props.file.program.name.clone()}</td>
            //   <td>{"parent_file_uuid"}</td>
            //   <td>{self.props.file.parent_file_uuid.clone()}</td>
          // <td>{"Upload by:"}</td>
          <td>{format!("{} {} (@{})",
            self.props.file.owner_user.firstname.clone(),
            self.props.file.owner_user.lastname.clone(),
            self.props.file.owner_user.username.clone(),
          )}</td>
            //   <td>{"Created at:"}</td>
            //   <td>{format!("{:.*}", 19, self.props.file.created_at.to_string())}</td>
          // <td>{"Upload at:"}</td>
          <td>{format!("{:.*}", 19, self.props.file.updated_at.to_string())}</td>
        </tr>}
    }
}
