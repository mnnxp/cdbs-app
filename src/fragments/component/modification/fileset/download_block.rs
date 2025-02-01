use yew::{Component, Callback, ComponentLink, Html, Properties, ShouldRender, html, ChangeData};
use log::debug;
use crate::types::{UUID, FilesetProgramInfo};
use crate::services::get_value_field;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub select_modification_uuid: UUID,
    pub current_filesets_program: Vec<FilesetProgramInfo>,
    pub callback_select_fileset_uuid: Callback<UUID>,
    pub callback_open_fileset: Callback<bool>,
}

pub struct ManageFilesOfFilesetBlock {
    props: Props,
    link: ComponentLink<Self>,
    select_fileset_uuid: UUID,
    open_fileset_files_card: bool,
}

pub enum Msg {
    ParseFirstFilesetUuid,
    SelectFilesetUuid(UUID),
    OpenFilesetFilesBlock,
}

impl Component for ManageFilesOfFilesetBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            select_fileset_uuid: String::new(),
            open_fileset_files_card: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ParseFirstFilesetUuid => {
                self.select_fileset_uuid = self.props.current_filesets_program
                    .first()
                    .map(|fd| {
                        debug!("mod fileset_uuid: {:?}", fd.uuid);
                        debug!("mod program_name: {:?}", fd.program.name);
                        fd.uuid.clone()
                    })
                    .unwrap_or_default();
                self.props.callback_select_fileset_uuid.emit(self.select_fileset_uuid.clone());
            },
            Msg::SelectFilesetUuid(fileset_uuid) => {
                self.props.callback_select_fileset_uuid.emit(fileset_uuid.clone());
                self.select_fileset_uuid = fileset_uuid;
            },
            Msg::OpenFilesetFilesBlock => {
                self.open_fileset_files_card = !self.open_fileset_files_card;
                self.props.callback_open_fileset.emit(self.open_fileset_files_card);
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.select_modification_uuid == props.select_modification_uuid &&
            self.props.current_filesets_program.len() == props.current_filesets_program.len() {
            debug!("no change download block: {:?}", props.select_modification_uuid);
            false
        } else {
            debug!("change download block: {:?}", props.select_modification_uuid);
            self.props = props;
            self.link.send_message(Msg::ParseFirstFilesetUuid);
            true
        }
    }

    fn view(&self) -> Html {
        let onchange_select_fileset_btn =
            self.link.callback(|ev: ChangeData| Msg::SelectFilesetUuid(match ev {
                ChangeData::Select(el) => el.value(),
                _ => String::new(),
            }));
        let onclick_open_fileset_files_list_btn =
            self.link.callback(|_| Msg::OpenFilesetFilesBlock);
        let class_fileset_btn = match self.open_fileset_files_card {
            true => "button is-info is-light is-active",
            false => "button is-info",
        };
        html!{
            <div style="margin-right: .5rem">
                <div class="select" style="margin-right: .5rem">
                    <select
                    id={"select-fileset-program-download"}
                    select={self.select_fileset_uuid.clone()}
                    onchange={onchange_select_fileset_btn}
                    title={get_value_field(&207)}>
                        {for self.props.current_filesets_program.iter().map(|fd|
                            html!{
                                <option value={fd.uuid.to_string()}
                                        selected={fd.uuid == self.select_fileset_uuid} >
                                    {fd.program.name.clone()}
                                </option>
                            }
                        )}
                    </select>
                </div>
                <button
                class={class_fileset_btn}
                onclick={onclick_open_fileset_files_list_btn}
                title={get_value_field(&106)}>
                    <span class={"icon is-small"}><i class={"fa fa-list"}></i></span>
                    <span>{get_value_field(&198)}</span>
                </button>
            </div>
        }
    }
}