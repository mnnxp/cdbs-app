mod item;

pub use item::FileItem;

use yew::{Component, ComponentLink, Html, Properties, ShouldRender, html};
// use log::debug;
// use crate::error::{get_error, Error};
use crate::types::{UUID, ShowFileInfo};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub show_download_btn: bool,
    pub show_delete_btn: bool,
    pub component_uuid: UUID,
    pub files: Vec<ShowFileInfo>,
}

pub struct FilesCard {
    link: ComponentLink<Self>,
    props: Props,
    show_full_files: bool,
}

#[derive(Clone)]
pub enum Msg {
    ShowFullList,
    Ignore,
}

impl Component for FilesCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            show_full_files: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // let link = self.link.clone();
        match msg {
            Msg::ShowFullList => self.show_full_files = !self.show_full_files,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.component_uuid == props.component_uuid &&
            self.props.show_download_btn == props.show_download_btn &&
                self.props.show_delete_btn == props.show_delete_btn &&
                    self.props.files.len() == props.files.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="files" class="card">
                {for self.props.files.iter().enumerate().map(|(index, file)| {
                    match (index >= 3, self.show_full_files) {
                        // show full list
                        (_, true) => html! {<FileItem
                          show_download_btn = self.props.show_download_btn.clone()
                          show_delete_btn = self.props.show_delete_btn.clone()
                          component_uuid = self.props.component_uuid.clone()
                          file = file.clone()
                        />},
                        // show full list or first 4 items
                        (false, false) => html! {<FileItem
                          show_download_btn = self.props.show_download_btn.clone()
                          show_delete_btn = self.props.show_delete_btn.clone()
                          component_uuid = self.props.component_uuid.clone()
                          file = file.clone()
                        />},
                        _ => html!{},
                    }
                })}
                {match self.props.files.len() {
                    0 => html!{<span>{"Files not found"}</span>},
                    0..=3 => html!{},
                    _ => self.show_see_btn(),
                }}
            </div>
        }
    }
}

impl FilesCard {
    fn show_see_btn(&self) -> Html {
        let show_full_files_btn = self.link
            .callback(|_| Msg::ShowFullList);

        match self.show_full_files {
            true => html!{<>
              <button class="button is-white"
                  onclick=show_full_files_btn
                >{"See less"}</button>
            </>},
            false => html!{<>
              <button class="button is-white"
                  onclick=show_full_files_btn
                >{"See more"}</button>
            </>},
        }
    }
}
