use yew::{Component, ComponentLink, Html, InputData, ChangeData, Properties, ShouldRender, html, Callback};
use log::debug;
use crate::types::PaginateSet;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub callback_change: Callback<PaginateSet>,
    pub current_items: i64,
    pub current_page: Option<i64>,
    pub per_page: Option<i64>,
    pub total_items: Option<i64>,
}

pub struct Paginate {
    link: ComponentLink<Self>,
    props: Props,
    new_page: i64,
    page_set: PaginateSet,
    show_options: [i64;10],
    is_manual_page: bool,
    total_page: i64,
}

#[derive(Clone)]
pub enum Msg {
    ToFirst,
    Previous,
    Next,
    ToLast,
    To(String),
    ChangePerPage(String),
    SetPage,
}

impl Component for Paginate {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let page_set = PaginateSet::set(props.current_page, props.per_page);
        Self {
            link,
            props,
            new_page: page_set.current_page,
            page_set,
            show_options: [3,5,10,20,30,40,50,75,100,500],
            is_manual_page: false,
            total_page: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update_total_page();
            self.props.callback_change.emit(self.page_set.clone());
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToFirst => self.page_set.to(1),
            Msg::Previous => self.page_set.previous(),
            Msg::Next => self.page_set.next(),
            Msg::ToLast => self.page_set.to(self.total_page),
            Msg::To(number) => self.new_page = number.parse::<i64>().unwrap_or_default(),
            Msg::ChangePerPage(value) => {
                if let Ok(per_page) = value.parse::<i64>() {
                    self.page_set.max_on_page(per_page)
                }
                self.update_total_page();
            },
            Msg::SetPage => {
                self.is_manual_page = !self.is_manual_page;
                if self.is_manual_page {
                    self.new_page = self.page_set.current_page;
                }
                if self.new_page > 0 {
                    self.page_set.to(self.new_page);
                }
            },
        }
        debug!("Update PaginateSet {:?}", self.page_set);
        // do not send the request while is manually entering the page number
        if !self.is_manual_page {
            self.props.callback_change.emit(self.page_set.clone());
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.current_items == props.current_items &&
        self.props.per_page == props.per_page {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let color = "color: #1872f0;";
        html!{
            <nav class="pagination" role="navigation" aria-label="pagination">
                {self.ft_previous_btn(color)}
                {self.ft_next_btn(color)}
                {self.ft_per_page()}
                {match self.is_manual_page {
                    true => self.ft_current_page(),
                    false => self.ft_total_items(),
                }}
            </nav>
        }
    }
}

impl Paginate {
    fn ft_total_items(&self) -> Html {
        let onclick_set_page = self.link.callback(|_| Msg::SetPage);
        html!{
            <div class="button pagination-link"
                    aria-label={self.page_set.current_page.to_string()}
                    aria-current={self.page_set.current_page.to_string()}
                    onclick={onclick_set_page}
                    >
                {self.page_set.current_page.to_string()}
                {"/"}
                {self.total_page.to_string()}
            </div>
        }
    }

    fn ft_current_page(&self) -> Html {
        let onclick_set_page = self.link.callback(|_| Msg::SetPage);
        let oninput_change_page = self.link.callback(|ev: InputData| Msg::To(ev.value));
        html!{<>
            <input
                id={"current_page"}
                class={"pagination-link"}
                type={"number"}
                min="1"
                max={self.total_page.to_string()}
                placeholder={self.page_set.current_page.to_string()}
                value={self.new_page.to_string()}
                oninput={oninput_change_page} />
            <button
                id={"to"}
                class={"button is-link pagination-link"}
                onclick={onclick_set_page}>
                <span class="icon">
                    <i class="fas fa-space-shuttle" aria-hidden="true"></i>
                </span>
            </button>
        </>}
    }

    fn ft_previous_btn(&self, color: &str) -> Html {
        let onclick_previous = self.link.callback(|_| Msg::Previous);
        let onclick_first = self.link.callback(|_| Msg::ToFirst);
        let disabled = self.page_set.current_page == 1 || self.is_manual_page;
        let color = color.to_string();
        html!{<>
            <button
                id={"to_first"}
                class={"button pagination-previous"}
                onclick={onclick_first}
                disabled={disabled}>
                <span class="icon">
                    <i class="fas fa-step-backward" aria-hidden="true" style={color.clone()}></i>
                </span>
            </button>
            <button
                id={"to_previous"}
                class={"button pagination-previous"}
                onclick={onclick_previous}
                disabled={disabled}>
                <span class="icon">
                    <i class="fas fa-caret-left" aria-hidden="true" style={color}></i>
                </span>
            </button>
        </>}
    }

    fn ft_next_btn(&self, color: &str) -> Html {
        let onclick_next = self.link.callback(|_| Msg::Next);
        let onclick_last = self.link.callback(|_| Msg::ToLast);
        let disabled = self.page_set.current_page >= self.total_page || self.is_manual_page;
        let color = color.to_string();
        html!{<>
            <button
                id={"to_next"}
                class={"button pagination-next"}
                onclick={onclick_next}
                disabled={disabled}>
                <span class="icon">
                    <i class="fas fa-caret-right" aria-hidden="true" style={color.clone()}></i>
                </span>
            </button>
            <button
                id={"to_last"}
                class={"button pagination-next"}
                onclick={onclick_last}
                disabled={disabled}>
                <span class="icon">
                    <i class="fas fa-step-forward" aria-hidden="true" style={color}></i>
                </span>
            </button>
        </>}
    }

    fn ft_per_page(&self) -> Html {
        let onchange_change_per_page = self.link.callback(|ev: ChangeData| Msg::ChangePerPage(match ev {
            ChangeData::Select(el) => el.value(),
            _ => "1".to_string(),
        }));

        html!{
            <div class={"select"}>
                <select
                    id={"per_page"}
                    select={self.page_set.per_page.to_string()}
                    onchange={onchange_change_per_page} >
                    {for self.show_options.iter().map(|so|
                        html!{
                            <option value={so.to_string()} selected={so == &self.page_set.per_page} >
                                {&so}
                            </option>
                        }
                    )}
                </select>
            </div>
        }
    }

    fn update_total_page(&mut self) {
        debug!("Update total page: {:?}, {}", self.props.total_items, self.page_set.per_page);
        // is set to 1 if there are fewer items than shown on page 1 or they are not specified
        self.total_page = match self.props.total_items {
            Some(ti) if ti > self.page_set.per_page => {
                debug!("Update total page %={:?}", ti%self.page_set.per_page);
                if ti%self.page_set.per_page > 0 {
                    ti/self.page_set.per_page+1
                } else {
                    ti/self.page_set.per_page
                }
            },
            _ => 1,
        }
    }
}