use yew::{
    html, Callback, Component, ComponentLink,
    Html, Properties, ShouldRender,
};
// use log::debug;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub param_id: usize,
    pub value: Option<String>,
    pub callback_change_param: Option<Callback<usize>>,
}

pub struct ModificationTableItemModule {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ChangeParam,
}

impl Component for ModificationTableItemModule {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeParam => {
                if let Some(rollback) = &self.props.callback_change_param {
                    rollback.emit(self.props.param_id);
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.param_id == props.param_id &&
              self.props.value == props.value {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        match self.props.callback_change_param {
            Some(_) => match self.props.param_id {
                0 => self.show_column_for_add(),
                _=> self.with_manage_btn(),
            },
            None => self.without_manage_btn(),
        }
    }
}

impl ModificationTableItemModule {
    fn show_column_for_add(&self) -> Html {
        let onclick_new_param = self.link.callback(|_| Msg::ChangeParam);

        html!{<td>
            <a class="button is-success is-rounded is-small is-light"
                  onclick={onclick_new_param.clone()} >
                <span class="icon" >
                  <i class="fa fa-plus" aria-hidden="true"></i>
                </span>
            </a>
        </td>}
    }

    fn with_manage_btn(&self) -> Html {
        let onclick_change_param = self.link.callback(|_| Msg::ChangeParam);

        html!{<td>
            <a onclick={onclick_change_param.clone()}>
                {match &self.props.value {
                    Some(value) => html!{<>
                        <span>{value.clone()}</span>
                        <span class="icon is-small" >
                          <i class="fas fa-pen" aria-hidden="true"></i>
                        </span>
                    </>},
                    None => html!{<span class="icon" >
                      <i class="fa fa-plus" aria-hidden="true"></i>
                    </span>},
                }}
            </a>
        </td>}
    }

    fn without_manage_btn(&self) -> Html {
        match &self.props.value {
            Some(value) => html!{<td>{value.clone()}</td>},
            None => html!{<td></td>},
        }
    }
}
