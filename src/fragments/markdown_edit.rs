use yew::{Callback, html, Properties, Component, ComponentLink, Html, ShouldRender, InputData};
use crate::services::get_value_field;
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
  PreviewDescription,
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
            Msg::PreviewDescription => self.is_preview = !self.is_preview,
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
        let (text_btn, style_preview, style_raw) = match self.is_preview {
            true => (get_value_field(&334), "", "display: none;"),
            false => (get_value_field(&335), "display: none;", ""),
        };
        html!{
            <div id={format!("md-edit-{}", self.props.id_tag)} class={"column"}>
                <label class={"title is-5"} for="markdown-raw">{self.props.title.clone()}</label>
                <div class="card is-shadowless" style="border: 1px solid #dbdbdb">
                    <header class="card-header">
                        <button class="card-footer-item button is-white is-small" onclick={self.link.callback(|_| Msg::PreviewDescription)}>{text_btn}</button>
                    </header>
                    <div class="card-content p-0">
                        <div class="content">
                            <div id={"markdown-preview"} style={style_preview} class={"p-2"}>
                                {self.props.raw_text.to_markdown()}
                            </div>
                            <textarea
                                id={"markdown-raw"}
                                class={"textarea is-fullwidth"}
                                style={style_raw}
                                type={"text"}
                                placeholder={self.props.placeholder.clone()}
                                value={self.props.raw_text.clone()}
                                oninput={self.props.oninput_text.clone()} />
                        </div>
                    </div>
                    <footer class="card-footer">
                        <p class="help p-1">{get_value_field(&336)}</p>
                    </footer>
                </div>
            </div>
        }
    }
}