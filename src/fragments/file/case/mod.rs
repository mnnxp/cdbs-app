use std::collections::BTreeSet;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender, Callback};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::types::{ShowFileInfo, UUID};
use crate::services::{Size, get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::relate::{
  ShowFileRevisions, show_file_revisions,
  ChangeActiveFileRevision, change_active_file_revision,
};

pub struct FileShowcase {
  error: Option<Error>,
  download_url: String,
  props: Props,
  link: ComponentLink<Self>,
  file_arr: Vec<ShowFileInfo>,
  active_revision: UUID,
  files_deleted_list: BTreeSet<UUID>,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub file_info: ShowFileInfo,
  pub file_info_callback: Callback<()>,
  pub file_download_callback: Option<Callback<UUID>>,
  pub file_delete_callback: Option<Callback<UUID>>,
  pub open_modal_frame: bool,
  pub show_revisions: bool,
  pub download_url: String,
}

#[derive(Clone)]
pub enum Msg {
  RequestRevisionsFile(UUID),
  ParsingFiles(String),
  SetActiveRev(UUID),
  GetActiveRevResult(String, String),
  ClickFileInfo,
  ClickDownloadFile,
  ClickDeleteFile(UUID),
  ResponseError(Error),
  ClearError,
  Ignore,
}

impl Component for FileShowcase {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      let active_revision = props.file_info.uuid.clone();
      FileShowcase {
        error: None,
        download_url: props.download_url.clone(),
        props,
        link,
        file_arr: Vec::new(),
        active_revision,
        files_deleted_list: BTreeSet::new(),
      }
    }

    fn rendered(&mut self, first_render: bool) {
      // if first_render && self.props.show_revisions && !self.active_revision.is_empty() {
      if first_render && !self.props.file_info.uuid.is_empty() {
        // self.link.send_message(Msg::RequestRevisionsFile(self.active_revision.clone()));

        if self.props.show_revisions {
          self.active_revision = self.props.file_info.uuid.clone();
          self.files_deleted_list.clear();
          self.link.send_message(Msg::RequestRevisionsFile(self.active_revision.clone()));
        }

        if self.download_url.is_empty() {
          self.link.send_message(Msg::ClickDownloadFile);
        }
      }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      let link = self.link.clone();

      match msg {
        Msg::RequestRevisionsFile(file_uuid) => {
          spawn_local(async move {
              let res = make_query(ShowFileRevisions::build_query(show_file_revisions::Variables{
                file_uuid,
                limit: None,
                offset: None,
              })).await.unwrap();
              link.send_message(Msg::ParsingFiles(res));
          })
        },
        Msg::ParsingFiles(res) => {
          match resp_parsing(res, "showFileRevisions") {
            Ok(file_arr) => {
              // debug!("showFileRevisions: {:?}", file_arr);
              self.file_arr = file_arr;
            },
            Err(err) => link.send_message(Msg::ResponseError(err)),
          }
        },
        Msg::SetActiveRev(file_uuid) => {
          spawn_local(async move {
            let res = make_query(ChangeActiveFileRevision::build_query(change_active_file_revision::Variables{
              file_uuid: file_uuid.clone(),
            })).await.unwrap();
            link.send_message(Msg::GetActiveRevResult(res, file_uuid));
          })
        },
        Msg::GetActiveRevResult(res, file_uuid) => {
          match resp_parsing(res, "changeActiveFileRevision") {
            Ok(res) => {
              debug!("changeActiveFileRevision {:?}: {:?}", file_uuid, res);
              if res {
                self.active_revision = file_uuid;
                self.link.send_message(Msg::ClickDownloadFile);
              }
            },
            Err(err) => link.send_message(Msg::ResponseError(err)),
          }
        },
        Msg::ClickFileInfo => {
          self.props.open_modal_frame = !self.props.open_modal_frame;
          self.props.file_info_callback.emit(())
        },
        Msg::ClickDownloadFile => {
          if let Some(fd_callback) = &self.props.file_download_callback {
            fd_callback.emit(self.active_revision.clone());
            // self.download_url.clear()
          }
        },
        Msg::ClickDeleteFile(file_uuid) => {
          if let Some(fd_callback) = &self.props.file_delete_callback {
            fd_callback.emit(file_uuid.clone());
            self.files_deleted_list.insert(file_uuid);
          }
        },
        Msg::ResponseError(err) => self.error = Some(err),
        Msg::ClearError => self.error = None,
        Msg::Ignore => {}
      };
      true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if self.props.open_modal_frame == props.open_modal_frame &&
          self.props.show_revisions == props.show_revisions {
        debug!("show revisions: {:?}", self.props.show_revisions);
        if &self.download_url != &props.download_url {
          self.download_url = props.download_url.clone();
          return true
        }
        false
      } else {
        self.props = props;
        debug!("show revisions: {:?}", self.props.show_revisions);
        true
      }
    }

    fn view(&self) -> Html {
      let onclick_file_info = self.link.callback(|_| Msg::ClickFileInfo);
      let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
      let class_modal = match &self.props.open_modal_frame {
        true => "modal is-active",
        false => "modal",
      };

      html!{
        <div class=class_modal>
          <div class="modal-background" onclick=onclick_file_info.clone() />
          <div class="modal-content box">
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error}/>
            // <div class="card column">
              {match self.props.show_revisions {
                true => self.show_revisions(),
                false => self.show_info_card(self.props.file_info.uuid.clone()),
              }}
            // </div>
          </div>
          <button class="modal-close is-large" aria-label="close" onclick=onclick_file_info />
        </div>
      }
    }
}

impl FileShowcase {
  /// Showing file in table with revisions
  fn show_revisions(&self) -> Html {
    html!{<>
      <h1 class="title">
        <p>{self.props.file_info.filename.clone()}</p>
      </h1>
      <table class="table is-striped is-narrow is-hoverable is-fullwidth">
        <thead>{self.set_title()}</thead>
        // <tfoot>{self.set_title()}</tfoot>
        <tbody>
          {for self.file_arr.iter().map(|file_info| {
            match self.files_deleted_list.get(&file_info.uuid) {
              Some(_) => html!{}, // removed file
              None => self.tr_revision_generate(file_info, &self.active_revision == &file_info.uuid),
            }
          })}
        </tbody>
      </table>
    </>}
  }

  /// Sets title for table with revisions file
  fn set_title(&self) -> Html {
    html!{
      <tr>
        <th><abbr title={get_value_field(&308)}>{get_value_field(&309)}</abbr></th>
        // <th>{get_value_field(&236)}</th> // Filename
        // <th>{get_value_field(&237)}</th> // Content type
        <th><abbr title={get_value_field(&238)}>{get_value_field(&310)}</abbr></th> // Filesize
        // <th>{get_value_field(&239)}</th> // Program
        <th><abbr title={get_value_field(&240)}>{get_value_field(&311)}</abbr></th> // Upload by
        <th><abbr title={get_value_field(&242)}>{get_value_field(&312)}</abbr></th> // Created at
        // <th><abbr title={get_value_field(&241)}>{get_value_field(&313)}</abbr></th> // Upload at
        <th>{get_value_field(&111)}</th>
      </tr>
    }
  }

  fn tr_revision_generate(
    &self,
    file_info: &ShowFileInfo,
    select: bool
  ) -> Html {
    let select_str = match select {
      true => "is-selected",
      false => "",
    };
    html!{
      <tr class={select_str}>
        // <td>{file_info.revision.clone()}</td>
        <td><abbr title={file_info.uuid.clone()}>{file_info.revision.clone()}</abbr></td>
        // <td>{file_info.filename.clone()}</td>
        // <td>{file_info.content_type.clone()}</td>
        <td>{file_info.show_size()}</td>
        // <td>{file_info.program.name.clone()}</td>
        // <td>{file_info.parent_file_uuid.clone()}</td>
        <td>{format!("{} {} (@{})",
          file_info.owner_user.firstname.clone(),
          file_info.owner_user.lastname.clone(),
          file_info.owner_user.username.clone(),
        )}</td>
        <td>{format!("{:.*}", 19, file_info.created_at.to_string())}</td>
        // <td>{format!("{:.*}", 19, file_info.updated_at.to_string())}</td>
        <td>{match select_str.is_empty() {
          true => html!{
            <div class="buttons">
              {self.show_set_active_btn(file_info.uuid.clone())}
              {self.show_delete_btn(file_info.uuid.clone())}
            </div>
          },
          false => html!{
            <div class="buttons">
              // {self.show_set_active_btn(file_info.uuid.clone())}
              {self.show_download_btn()}
              {self.show_delete_btn(file_info.uuid.clone())}
            </div>
          },
        }}</td>
      </tr>
    }
  }

  fn show_info_card(&self, file_uuid: UUID) -> Html {
    let onclick_delete_btn =
      self.link.callback(move |_| Msg::ClickDeleteFile(file_uuid.clone()));
    html!{<>
      {self.show_full_info_file()}
      {match &self.props.file_delete_callback.is_some() {
        true => html!{
          <div class="buttons">
            {self.show_download_full_btn()}
            <button class="button is-white is-danger is-fullwidth" onclick=onclick_delete_btn >
              <span class="icon" >
                <i class="fa fa-trash" aria-hidden="true"></i>
              </span>
              <span>{get_value_field(&135)}</span>
            </button>
          </div>
        },
        false => {self.show_download_full_btn()},
      }}
    </>}
  }

  /// Show modal with info about a file
  fn show_full_info_file(&self) -> Html {
    html!{
      <div class="card column">
        <table class="table is-fullwidth">
          <tbody>
            <tr>
              <td>{get_value_field(&236)}</td> // Filename
              <td>{self.props.file_info.filename.clone()}</td>
            </tr>
            <tr>
              <td>{get_value_field(&308)}</td> // Revision
              <td>{self.props.file_info.revision}</td>
            </tr>
            // <tr>
            //   <td>{get_value_field(&237)}</td> // Content type
            //   <td>{self.props.file_info.content_type.clone()}</td>
            // </tr>
            <tr>
              <td>{get_value_field(&238)}</td> // Filesize
              <td>{self.props.file_info.show_size()}</td>
            </tr>
            // <tr>
            //   <td>{get_value_field(&239)}</td> // Program
            //   <td>{self.props.file_info.program.name.clone()}</td>
            // </tr>
            <tr>
              <td>{get_value_field(&240)}</td> // Upload by
              <td>{format!("{} {} (@{})",
                self.props.file_info.owner_user.firstname.clone(),
                self.props.file_info.owner_user.lastname.clone(),
                self.props.file_info.owner_user.username.clone(),
              )}</td>
            </tr>
            <tr>
              <td>{get_value_field(&242)}</td> // Created at
              <td>{format!("{:.*}", 19, self.props.file_info.created_at.to_string())}</td>
            </tr>
            // <tr>
            //   <td>{get_value_field(&241)}</td> // Upload at
            //   <td>{format!("{:.*}", 19, self.props.file_info.updated_at.to_string())}</td>
            // </tr>
          </tbody>
        </table>
      </div>
    }
  }

  fn show_download_btn(&self) -> Html {
    html!{
      <a class="button is-white"
          href={self.download_url.clone()}
          disabled={self.download_url.is_empty()}
          target="_blank"
          >
        <span class="icon" >
          <i class="fas fa-file-download" style="color: #1872f0;" aria-hidden="true"></i>
        </span>
      </a>
    }
  }

  fn show_download_full_btn(&self) -> Html {
    html!{
      <a class="button is-info is-fullwidth"
          href={self.download_url.clone()}
          disabled={self.download_url.is_empty()}
          target="_blank"
          >
        <span class="icon" >
          <i class="fas fa-file-download" aria-hidden="true"></i>
        </span>
        <span>{get_value_field(&126)}</span>
      </a>
    }
  }

  fn show_set_active_btn(&self, file_uuid: UUID) -> Html {
    let onclick_set_active_btn = self.link.callback(move |_| Msg::SetActiveRev(file_uuid.clone()));

    match &self.props.file_delete_callback.is_some() {
      true => html!{
        <button class="button is-white is-responsive" onclick=onclick_set_active_btn >
          <span class="icon" >
            <i class="fas fa-angle-double-left" style="color: #1872f0;" aria-hidden="true"></i>
          </span>
        </button>
      },
      false => html!{},
    }
  }

  fn show_delete_btn(&self, file_uuid: UUID) -> Html {
    let onclick_delete_btn =
      self.link.callback(move |_| Msg::ClickDeleteFile(file_uuid.clone()));

    match &self.props.file_delete_callback.is_some() {
      true => html!{
        <button class="button is-white is-danger is-inverted is-responsive" onclick=onclick_delete_btn >
          <span class="icon" >
            <i class="fa fa-trash" aria-hidden="true"></i>
          </span>
        </button>
      },
      false => html!{},
    }
  }
}
