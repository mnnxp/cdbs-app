use yew::{Callback, html, Properties, Component, ComponentLink, Html, ShouldRender, InputData};
use crate::services::{get_value_field, unique_id};
use crate::services::content_adapter::Markdownable;

pub struct MarkdownEditCard {
    props: Props,
    link: ComponentLink<Self>,
    is_preview: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id_tag: String,
    pub title: String,
    pub placeholder: String,
    pub raw_text: String,
    pub oninput_text: Callback<InputData>,
}

pub enum Msg {
  SetPreview(bool),
}

impl Component for MarkdownEditCard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MarkdownEditCard {
            props,
            link,
            is_preview: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetPreview(state) => self.is_preview = state,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.raw_text == props.raw_text {
            false
        } else {
            self.props = props;
            self.is_preview = false;
            true
        }
    }

    fn view(&self) -> Html {
        let textarea_id = unique_id("markdown-raw");
        let (tab_write_class, tab_preview_class) = match self.is_preview {
            true => ("", "is-active"),
            false => ("is-active", ""),
        };

        html!{
            <div id={unique_id(&self.props.id_tag)}>
                <div class="mb-2">
                    <label class="label mb-0" for={textarea_id.clone()}>
                        {self.props.title.clone()}
                    </label>
                </div>
                <div class="card is-shadowless card-bordered">
                    <header class="card-header px-3 is-shadowless">
                        <div class="tabs is-small m-0 p-0">
                            <ul class="m-0 p-0">
                                <li class={tab_write_class}>
                                    <a onclick={self.link.callback(|_| Msg::SetPreview(false))}>
                                        <span class="icon is-small"><i class="fas fa-edit"></i></span>
                                        <span>{get_value_field(&334)}</span>
                                    </a>
                                </li>
                                <li class={tab_preview_class}>
                                    <a onclick={self.link.callback(|_| Msg::SetPreview(true))}>
                                        <span class="icon is-small"><i class="fas fa-eye"></i></span>
                                        <span>{get_value_field(&335)}</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </header>
                    <div class="card-content p-0">
                        {
                            if self.is_preview {
                                html!{
                                    <div id="markdown-preview" class="content p-4 block">
                                        {self.props.raw_text.to_markdown()}
                                    </div>
                                }
                            } else {
                                html!{<>
                                    <textarea
                                        id={textarea_id}
                                        class="textarea is-fullwidth is-shadowless p-4"
                                        placeholder={self.props.placeholder.clone()}
                                        value={self.props.raw_text.clone()}
                                        oninput={self.props.oninput_text.clone()} />
                                    <div class="help has-text-grey-light ml-1">
                                        <span class="icon is-small mr-1"><i class="fas fa-info-circle"></i></span>
                                        <span>{get_value_field(&336)}</span>
                                    </div>
                                </>}
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }
}