use yew::{html, Component, ComponentLink, Html, ShouldRender, Callback, Properties};

use crate::{services::get_value_field, types::TypeAccessInfo};


pub struct TypeAccessBlock {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub change_cb: Callback<usize>,
    pub types: Vec<TypeAccessInfo>,
    pub selected: usize,
    pub preset: Option<usize>,
}

#[derive(Clone)]
pub enum Msg {
    UpdateTypeAccessId(usize),
    Ignore,
}

impl Component for TypeAccessBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TypeAccessBlock {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTypeAccessId(value) => self.props.change_cb.emit(value),
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.selected == props.selected &&
            self.props.types.len() == props.types.len() {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html!{
              <div class="control">
                { for self.props.types.iter().map(|x| {
                    let type_access_id = x.type_access_id;
                    html!{
                      <div class="column p-0 m-0">
                      <label class="radio">
                          <input
                              id="type-access-block"
                              type="radio"
                              name="type-access"
                              value={x.type_access_id.to_string()}
                              onchange={self.link.callback(move |_| Msg::UpdateTypeAccessId(type_access_id))}
                              checked={match self.props.selected {
                                0 => x.type_access_id == self.props.preset.unwrap_or_default(),
                                _ => x.type_access_id == self.props.selected,
                              }}
                          />
                          <span>{" "}{x.get_with_icon()}{": "}<em>{get_value_field(&(397+x.type_access_id))}</em></span>
                      </label>
                      </div>
                    }
                })}
          </div>
        }
    }
}
