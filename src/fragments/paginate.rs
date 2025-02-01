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
    CalcPages,
    ToFirst,
    Prev,
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
            show_options: [1,3,5,10,20,30,40,50,75,100],
            is_manual_page: false,
            total_page: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::CalcPages);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CalcPages => self.update_total_page(),
            Msg::ToFirst => self.page_set.to(1),
            Msg::Prev => self.page_set.previous(),
            Msg::Next => self.page_set.next(),
            Msg::ToLast => self.page_set.to(self.total_page),
            Msg::To(number) => self.new_page = number.parse::<i64>().unwrap_or_default(),
            Msg::ChangePerPage(value) => {
                if let Ok(per_page) = value.parse::<i64>() {
                    self.page_set.max_on_page(per_page);
                    self.page_set.to(1);
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
        if  self.props.current_items == props.current_items &&
        self.props.per_page == props.per_page &&
        self.props.total_items == props.total_items {
            false
        } else {
            self.props = props;
            self.link.send_message(Msg::CalcPages);
            true
        }
    }

    fn view(&self) -> Html {
        let color = "#1872f0";
        html!{
            <nav class={"pagination-smart"} role={"navigation"} aria-label={"pagination-smart"}>
                <div class={"buttons"}>
                    {self.ft_per_page()}
                    {self.ft_prev_btn(color)}
                    {match self.is_manual_page {
                        true => self.ft_current_page(),
                        false => self.ft_total_items(),
                    }}
                    {self.ft_next_btn(color)}
                </div>
            </nav>
        }
    }
}

impl Paginate {
    fn ft_total_items(&self) -> Html {
        let onclick_set_page = self.link.callback(|_| Msg::SetPage);
        let total_page = match self.total_page > 0 {
            true => format!("/{}", self.total_page),
            false => String::from("/?"),
        };
        html!{
            <div class={"button pagination-smart"}
                    aria-label={self.page_set.current_page.to_string()}
                    aria-current={self.page_set.current_page.to_string()}
                    onclick={onclick_set_page}
                    >
                {self.page_set.current_page.to_string()}
                {total_page}
            </div>
        }
    }

    fn ft_current_page(&self) -> Html {
        let onclick_set_page = self.link.callback(|_| Msg::SetPage);
        let oninput_change_page = self.link.callback(|ev: InputData| Msg::To(ev.value));
        html!{<>
            <input
                id={"current_page"}
                class={"input pagination-smart"}
                type={"number"}
                min={"1"}
                max={self.total_page.to_string()}
                placeholder={self.page_set.current_page.to_string()}
                value={self.new_page.to_string()}
                oninput={oninput_change_page} />
            <button
                id={"to"}
                class={"button is-link pagination-smart"}
                onclick={onclick_set_page}>
                <span class="icon">
                    <i class="fas fa-space-shuttle" aria-hidden="true"></i>
                </span>
            </button>
        </>}
    }

    fn ft_prev_btn(&self, color: &str) -> Html {
        let onclick_prev = self.link.callback(|_| Msg::Prev);
        let onclick_first = self.link.callback(|_| Msg::ToFirst);
        let disabled = self.page_set.current_page == 1 || self.is_manual_page;
        let color = color.to_string();
        html!{<>
            <button
                id={"to_first"}
                class={"button pagination-smart ps-first"}
                onclick={onclick_first}
                disabled={disabled}>
                <span class="icon">
                    <svg viewBox="0 0 36 36" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                        <path fill={color.clone()} d={"M7.08,6.52a1.68,1.68,0,0,0,0,2.4L16.51,18,7.12,27.08a1.7,1.7,0,0,0,2.36,2.44h0L21.4,18,9.48,6.47A1.69,1.69,0,0,0,7.08,6.52Z"}></path>
                        <path fill={color.clone()} d={"M26.49,5a1.7,1.7,0,0,0-1.7,1.7V29.3a1.7,1.7,0,0,0,3.4,0V6.7A1.7,1.7,0,0,0,26.49,5Z"}></path>
                    </svg>
                </span>
            </button>
            <button
                id={"to_prev"}
                class={"button pagination-smart ps-prev"}
                onclick={onclick_prev}
                disabled={disabled}>
                <span class="icon">
                    <svg viewBox="0 0 36 36" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                        <path fill={color} d={"M29.52,22.52,18,10.6,6.48,22.52a1.7,1.7,0,0,0,2.45,2.36L18,15.49l9.08,9.39a1.7,1.7,0,0,0,2.45-2.36Z"}></path>
                    </svg>
                </span>
            </button>
        </>}
    }

    fn ft_next_btn(&self, color: &str) -> Html {
        let onclick_next = self.link.callback(|_| Msg::Next);
        let onclick_last = self.link.callback(|_| Msg::ToLast);
        let disabled = match self.total_page == 0 {
            true => self.props.current_items < self.page_set.per_page,
            false => self.page_set.current_page >= self.total_page || self.is_manual_page,
        };
        let color = color.to_string();
        html!{<>
            <button
                id={"to_next"}
                class={"button pagination-smart ps-next"}
                onclick={onclick_next}
                disabled={disabled}>
                <span class="icon">
                    <svg viewBox="0 0 36 36" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                        <path fill={color.clone()} d={"M29.52,22.52,18,10.6,6.48,22.52a1.7,1.7,0,0,0,2.45,2.36L18,15.49l9.08,9.39a1.7,1.7,0,0,0,2.45-2.36Z"}></path>
                    </svg>
                </span>
            </button>
            <button
                id={"to_last"}
                class={"button pagination-smart"}
                onclick={onclick_last}
                disabled={disabled || self.total_page == 0}>
                <span class="icon">
                    <svg viewBox="0 0 36 36" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                        <path fill={color.clone()} d={"M7.08,6.52a1.68,1.68,0,0,0,0,2.4L16.51,18,7.12,27.08a1.7,1.7,0,0,0,2.36,2.44h0L21.4,18,9.48,6.47A1.69,1.69,0,0,0,7.08,6.52Z"}></path>
                        <path fill={color} d={"M26.49,5a1.7,1.7,0,0,0-1.7,1.7V29.3a1.7,1.7,0,0,0,3.4,0V6.7A1.7,1.7,0,0,0,26.49,5Z"}></path>
                    </svg>
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
            <div class={"select pagination-smart"}>
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
            None => 0,
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