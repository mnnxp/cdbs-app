use yew::{classes, html, Component, ComponentLink, Html, InputData, ShouldRender};
use crate::fragments::search::{CatalogSpec, SearchArg, SearchBar};
use crate::services::wraps_text;

#[derive(Clone)]
pub enum Msg {
    ChangeSpec(usize),
    ByParams,
    BySpecs,
    ByKeywords,
    OnlyFavorite,
    ForCompany(String),
    ForStandard(String),
    ForUser(String),
    ToggleCatalogSpec,
    ToggleCheckboxs,
    ToggleForObjects,
    Ignore,
}

pub struct SearchPage {
    link: ComponentLink<Self>,
    search_arg: SearchArg,
    catalog_spec_expanded: bool,
    checkboxs_expanded: bool,
    for_objects_expanded: bool,
}

impl Component for SearchPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SearchPage {
            link,
            search_arg: SearchArg::by_spec_id(1),
            catalog_spec_expanded: true,
            checkboxs_expanded: true,
            for_objects_expanded: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeSpec(spec_id) => self.search_arg.set_spec_id(spec_id),
            Msg::ByParams => self.search_arg.by_params = !self.search_arg.by_params,
            Msg::BySpecs => self.search_arg.by_specs = !self.search_arg.by_specs,
            Msg::ByKeywords => self.search_arg.by_keywords = !self.search_arg.by_keywords,
            Msg::OnlyFavorite => self.search_arg.favorite = !self.search_arg.favorite,
            Msg::ForCompany(company_uuid) => self.search_arg.company_uuid = wraps_text(company_uuid),
            Msg::ForStandard(standard_uuid) => self.search_arg.standard_uuid = wraps_text(standard_uuid),
            Msg::ForUser(user_uuid) => self.search_arg.user_uuid = wraps_text(user_uuid),
            Msg::ToggleCatalogSpec => self.catalog_spec_expanded = !self.catalog_spec_expanded,
            Msg::ToggleCheckboxs => self.checkboxs_expanded = !self.checkboxs_expanded,
            Msg::ToggleForObjects => self.for_objects_expanded = !self.for_objects_expanded,
            Msg::Ignore => {},
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div id={"search-page"} class={"column"}>
                // <div class={"column"}></div>
                <div class={"column"}>
                    <div class={"columns"}>
                        <div class={"column is-one-quarter"}>{self.filters()}</div>
                        <div class={"column"}>
                            <SearchBar search_arg={self.search_arg.clone()} />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl SearchPage {
    fn filters(&self) -> Html {
        let callback_select_spec = self.link.callback(|spec_id| Msg::ChangeSpec(spec_id));
        html!{
            <div class={"block"}>
                <div class={"column"}>
                    <CatalogSpec callback_select_spec={callback_select_spec} />
                </div>
                <div class={"column"}>{self.checkboxs()}</div>
                <div class={"column"}>{self.for_objects()}</div>
            </div>
        }
    }


    fn for_objects(&self) -> Html {
        let oninput_for_company = self.link.callback(|ev: InputData| Msg::ForCompany(ev.value));
        let oninput_for_standard = self.link.callback(|ev: InputData| Msg::ForStandard(ev.value));
        let oninput_for_user = self.link.callback(|ev: InputData| Msg::ForUser(ev.value));
        let onclick_toggle = self.link.callback(|_| Msg::ToggleForObjects);
        
        html!{
            <div class={"card"}>
                <div class={"column is-flex is-justify-content-space-between is-align-items-center pointer"} onclick={onclick_toggle}>
                    <p class={"title is-5 select-title"} style="margin-bottom: 0px;">{"For objects"}</p>
                    <span class="icon is-clickable">
                        <i class={classes!("fas", if self.for_objects_expanded { "fa-chevron-up" } else { "fa-chevron-down" })}></i>
                    </span>
                </div>
                {if self.for_objects_expanded {
                    html!{
                        <>
                            <label class="column pt-0 mt-0 checkbox">
                                {"for_company (uuid)"}
                                <input class="input is-small" type="text" value={self.search_arg.company_uuid.clone().unwrap_or_default()} oninput={oninput_for_company}/>
                            </label>
                            <label class="column pt-0 mt-0 checkbox">
                                {"for_standard (uuid)"}
                                <input class="input is-small" type="text" value={self.search_arg.standard_uuid.clone().unwrap_or_default()} oninput={oninput_for_standard}/>
                            </label>
                            <label class="column pt-0 mt-0 checkbox">
                                {"for_user (uuid)"}
                                <input class="input is-small" type="text" value={self.search_arg.user_uuid.clone().unwrap_or_default()} oninput={oninput_for_user}/>
                            </label>
                        </>
                    }
                } else {
                    html!{}
                }}
            </div>
        }
    }

    fn checkboxs(&self) -> Html {
        let onclick_by_params = self.link.callback(|_| Msg::ByParams);
        let onclick_by_specs = self.link.callback(|_| Msg::BySpecs);
        let onclick_by_keywords = self.link.callback(|_| Msg::ByKeywords);
        let onclick_only_favorite = self.link.callback(|_| Msg::OnlyFavorite);
        let onclick_toggle = self.link.callback(|_| Msg::ToggleCheckboxs);
        
        html!{
            <div class={"card"}>
                <div class={"column is-flex is-justify-content-space-between is-align-items-center pointer"} onclick={onclick_toggle}>
                    <p class={"title is-5 select-title"} style="margin-bottom: 0px;">{"Checkboxs"}</p>
                    <span class="icon is-clickable">
                        <i class={classes!("fas", if self.checkboxs_expanded { "fa-chevron-up" } else { "fa-chevron-down" })}></i>
                    </span>
                </div>
                {if self.checkboxs_expanded {
                    html!{
                        <>
                            <label class="column pt-0 mt-0 checkbox">
                                <input type="checkbox" checked={self.search_arg.by_params} onclick={onclick_by_params}/>
                                {" "}{"by_params (component params)"}
                            </label>
                            <label class="column pt-0 mt-0 checkbox">
                                <input type="checkbox" checked={self.search_arg.by_specs} onclick={onclick_by_specs}/>
                                {" "}{"by_specs"}
                            </label>
                            <label class="column pt-0 mt-0 checkbox">
                                <input type="checkbox" checked={self.search_arg.by_keywords} onclick={onclick_by_keywords}/>
                                {" "}{"by_keywords"}
                            </label>
                            <label class="column pt-0 mt-0 checkbox">
                                <input type="checkbox" checked={self.search_arg.favorite} onclick={onclick_only_favorite}/>
                                {" "}{"only_favorite"}
                            </label>
                        </>
                    }
                } else {
                    html!{}
                }}
            </div>
        }
    }
}