use yew::{function_component, Callback, html, Html, Properties};
use web_sys::{DragEvent, Event};
use crate::services::get_value_field;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<Event>,
    pub ondrop: Callback<DragEvent>,
    pub ondragover: Callback<DragEvent>,
    pub ondragenter: Callback<DragEvent>,
    pub input_id: String,
    #[prop_or("*".to_string())]
    pub accept: String,
    #[prop_or_default]
    pub multiple: bool,
    pub file_label: usize,
}

#[function_component(FilesFrame)]
pub fn files_frame(props: &Props) -> Html {
    html!{
        <label
          for={props.input_id}
          class="file-label"
          style="width: 100%; text-align: center"
        >
          <input
              id={props.input_id}
              class="file-input"
              type="file"
              accept={props.accept}
              multiple={props.multiple}
              onchange={props.onchange} />
          <span class="file-cta"
              ondrop={props.ondrop}
              ondragover={props.ondragover}
              ondragenter={props.ondragenter}
              >
            <span class="file-icon">
              <i class="fas fa-upload"></i>
            </span>
            <span class="file-label">{ get_value_field(&props.file_label) }</span>
          </span>
        </label>
    }
}
