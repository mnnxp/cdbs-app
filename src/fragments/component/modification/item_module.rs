use yew::{Component, Callback, Context, html, html::Scope, Html, Properties};
// use log::debug;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub param_id: usize,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub callback_change_param: Option<Callback<usize>>,
}

pub struct ModificationTableItemModule {
    param_id: usize,
    value: Option<String>,
}

pub enum Msg {
    ChangeParam,
}

impl Component for ModificationTableItemModule {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            param_id: ctx.props().param_id,
            value: ctx.props().value.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeParam => {
                if let Some(rollback) = &ctx.props().callback_change_param {
                    rollback.emit(ctx.props().param_id);
                }
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.param_id == ctx.props().param_id &&
              self.value == ctx.props().value {
            false
        } else {
            self.param_id = ctx.props().param_id;
            self.value = ctx.props().value.clone();
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().callback_change_param {
            Some(_) => match ctx.props().param_id {
                0 => self.show_column_for_add(ctx.link()),
                _=> self.with_manage_btn(ctx.link(), ctx.props()),
            },
            None => self.without_manage_btn(ctx.props()),
        }
    }
}

impl ModificationTableItemModule {
    fn show_column_for_add(
        &self,
        link: &Scope<Self>,
    ) -> Html {
        let onclick_new_param = link.callback(|_| Msg::ChangeParam);

        html!{<td>
            <a class="button is-success is-rounded is-small is-light"
                  onclick={onclick_new_param.clone()} >
                <span class="icon" >
                  <i class="fa fa-plus" aria-hidden="true"></i>
                </span>
            </a>
        </td>}
    }

    fn with_manage_btn(
        &self,
        link: &Scope<Self>,
        props: &Props,
    ) -> Html {
        let onclick_change_param = link.callback(|_| Msg::ChangeParam);

        html!{<td>
            <a onclick={onclick_change_param.clone()}>
                {match &props.value {
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

    fn without_manage_btn(
        &self,
        props: &Props,
    ) -> Html {
        match &props.value {
            Some(value) => html!{<td>{value.clone()}</td>},
            None => html!{<td></td>},
        }
    }
}
