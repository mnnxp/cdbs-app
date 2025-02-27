use std::collections::BTreeSet;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender, Callback};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;

use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::{ft_download_btn, ft_download_full_btn};
use crate::types::{ShowFileInfo, UUID};
use crate::services::content_adapter::{ContentDisplay, DateDisplay};
use crate::services::{Size, get_value_field, resp_parsing};
use crate::gqls::make_query;
use crate::gqls::relate::{
  ShowFileRevisions, show_file_revisions,
  ChangeActiveFileRevision, change_active_file_revision,
};

pub struct FileShowcase {
  error: Option<Error>,
  props: Props,
  link: ComponentLink<Self>,
  file_arr: Vec<ShowFileInfo>,
  active_revision: UUID,
  files_deleted_list: BTreeSet<UUID>,
  get_confirm: UUID,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub file_info: ShowFileInfo,
  pub file_info_callback: Callback<()>,
  pub file_delete_callback: Option<Callback<UUID>>,
  pub open_modal_frame: bool,
  pub show_revisions: bool,
}

#[derive(Clone)]
pub enum Msg {
  RequestRevisionsFile,
  ParsingFiles(String),
  SetActiveRev(UUID),
  GetActiveRevResult(String, String),
  ClickFileInfo,
  ClickDeleteFile(UUID),
  ResponseError(Error),
  ClearError,
  Ignore,
}

impl Component for FileShowcase {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      FileShowcase {
        error: None,
        props,
        link,
        file_arr: Vec::new(),
        active_revision: String::new(),
        files_deleted_list: BTreeSet::new(),
        get_confirm: String::new(),
      }
    }

    fn rendered(&mut self, first_render: bool) {
      if first_render {
        self.active_revision = self.props.file_info.uuid.clone();
        if self.props.show_revisions && self.props.open_modal_frame {
          self.link.send_message(Msg::RequestRevisionsFile);
        }
      }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      let link = self.link.clone();
      match msg {
        Msg::RequestRevisionsFile => {
          if self.active_revision.len() != 36 {
            return true
          }
          let file_uuid = self.active_revision.clone();
          spawn_local(async move {
              let res = make_query(ShowFileRevisions::build_query(
                show_file_revisions::Variables{file_uuid}
              )).await.unwrap();
              link.send_message(Msg::ParsingFiles(res));
          })
        },
        Msg::ParsingFiles(res) => {
          match resp_parsing(res, "showFileRevisions") {
            Ok(file_arr) => self.file_arr = file_arr,
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
              }
            },
            Err(err) => link.send_message(Msg::ResponseError(err)),
          }
        },
        Msg::ClickFileInfo => {
          self.props.open_modal_frame = !self.props.open_modal_frame;
          self.props.file_info_callback.emit(());
          self.get_confirm.clear();
        },
        Msg::ClickDeleteFile(file_uuid) => {
          if self.get_confirm == file_uuid {
            if let Some(fd_callback) = &self.props.file_delete_callback {
              fd_callback.emit(file_uuid.clone());
              self.files_deleted_list.insert(file_uuid);
              }
          } else {
            self.get_confirm = file_uuid;
          }
        },
        Msg::ResponseError(err) => self.error = Some(err),
        Msg::ClearError => self.error = None,
        Msg::Ignore => {}
      };
      true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      debug!("show revisions: {:?}", self.props.show_revisions);
      if self.props.open_modal_frame == props.open_modal_frame &&
          self.props.show_revisions == props.show_revisions {
        false
      } else {
        self.files_deleted_list.clear();
        if self.props.file_info.filename != props.file_info.filename {
          self.active_revision = props.file_info.uuid.clone();
        }
        self.props = props;
        if self.props.show_revisions && self.props.open_modal_frame {
          self.link.send_message(Msg::RequestRevisionsFile);
        }
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
        <div class={class_modal}>
          <div class="modal-background" onclick={onclick_file_info.clone()} />
          <div class="modal-content box">
            <ListErrors error={self.error.clone()} clear_error={onclick_clear_error}/>
            {match self.props.show_revisions {
              true => self.show_revisions(),
              false => self.show_info_card(self.props.file_info.uuid.clone()),
            }}
          </div>
          <button class="modal-close is-large" aria-label="close" onclick={onclick_file_info} />
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
        <th><abbr title={get_value_field(&338)}>{get_value_field(&341)}</abbr></th> // Message to change
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
        <td><abbr title={file_info.uuid.clone()}>{file_info.revision.clone()}</abbr></td>
        <td>{file_info.show_size()}</td>
        <td>{file_info.commit_msg.clone()}</td>
        <td>{file_info.owner_user.to_display()}</td>
        <td>{file_info.updated_at.date_to_display()}</td>
        <td>{match select_str.is_empty() {
          true => html!{
            <div class="buttons">
              {self.show_set_active_btn(file_info.uuid.clone())}
              {self.show_delete_btn(file_info.uuid.clone())}
            </div>
          },
          false => html!{
            <div class="buttons">
              {ft_download_btn(self.props.file_info.download_url.clone(), true)}
              {self.show_delete_btn(file_info.uuid.clone())}
            </div>
          },
        }}</td>
      </tr>
    }
  }

  fn show_info_card(&self, file_uuid: UUID) -> Html {
    html!{<>
      {self.show_full_info_file()}
      {match &self.props.file_delete_callback.is_some() {
        true => html!{
          <div class="buttons">
            {ft_download_full_btn(self.props.file_info.download_url.clone())}
            {self.show_delete_full_btn(file_uuid)}
          </div>
        },
        false => {ft_download_full_btn(self.props.file_info.download_url.clone())},
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
            <tr>
              <td>{get_value_field(&238)}</td> // Filesize
              <td>{self.props.file_info.show_size()}</td>
            </tr>
            <tr>
              <td>{get_value_field(&341)}</td> // Message
              <td>{self.props.file_info.commit_msg.clone()}</td>
            </tr>
            <tr>
              <td>{get_value_field(&240)}</td> // Upload by
              <td>{self.props.file_info.owner_user.to_display()}</td>
            </tr>
            <tr>
              <td>{get_value_field(&242)}</td> // Created at
              <td>{self.props.file_info.created_at.date_to_display()}</td>
            </tr>
          </tbody>
        </table>
      </div>
    }
  }

  fn show_set_active_btn(&self, file_uuid: UUID) -> Html {
    let onclick_set_active_btn = self.link.callback(move |_| Msg::SetActiveRev(file_uuid.clone()));

    match &self.props.file_delete_callback.is_some() {
      true => html!{
        <button class="button is-white" onclick={onclick_set_active_btn} >
          <span class="icon">
            <i class="fas fa-angle-double-left" style="color: #1872f0;" aria-hidden="true"></i>
          </span>
        </button>
      },
      false => html!{},
    }
  }

  fn show_delete_btn(&self, file_uuid: UUID) -> Html {
    let file_uuid_cl = file_uuid.clone();
    let onclick_delete_btn =
      self.link.callback(move |_| Msg::ClickDeleteFile(file_uuid.clone()));

    match (&self.props.file_delete_callback.is_some(), self.get_confirm == file_uuid_cl) {
      (true, true) => self.show_delete_full_btn(file_uuid_cl),
      (true, false) => html!{
        <button class="button is-danger is-inverted" onclick={onclick_delete_btn} >
          <span class="icon">
            <i class="fa fa-trash" aria-hidden="true"></i>
          </span>
        </button>
      },
      _ => html!{},
    }
  }

  fn show_delete_full_btn(&self, file_uuid: UUID) -> Html {
    let onclick_delete_btn =
      self.link.callback(move |_| Msg::ClickDeleteFile(file_uuid.clone()));

    match &self.props.file_delete_callback.is_some() {
      true => html!{
        <button class="button is-danger is-fullwidth" onclick={onclick_delete_btn} >
          <span class="icon">
            <i class="fa fa-trash" aria-hidden="true"></i>
          </span>
          <span>{get_value_field(&220)}</span>
        </button>
      },
      false => html!{},
    }
  }
}
