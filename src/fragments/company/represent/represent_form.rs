use yew::{classes, html, Callback, ChangeData, Html, InputData};
use crate::fragments::form_input::{render_form_input, InputConfig};
use crate::services::get_value_field;
use crate::types::{CompanyRepresentUpdateInfo, Region, RegisterCompanyRepresentInfo, RepresentationType};

pub(crate) trait RepresentFormValues {
    fn name(&self) -> String;
    fn phone(&self) -> String;
    fn address(&self) -> String;
    fn region_id(&self) -> usize;
    fn representation_type_id(&self) -> usize;
}

impl RepresentFormValues for RegisterCompanyRepresentInfo {
    fn name(&self) -> String { self.name.clone() }
    fn phone(&self) -> String { self.phone.clone() }
    fn address(&self) -> String { self.address.clone() }
    fn region_id(&self) -> usize { self.region_id }
    fn representation_type_id(&self) -> usize { self.representation_type_id }
}

impl RepresentFormValues for CompanyRepresentUpdateInfo {
    fn name(&self) -> String { self.name.clone().unwrap_or_default() }
    fn phone(&self) -> String { self.phone.clone().unwrap_or_default() }
    fn address(&self) -> String { self.address.clone().unwrap_or_default() }
    fn region_id(&self) -> usize { self.region_id.unwrap_or(1) as usize }
    fn representation_type_id(&self) -> usize { self.representation_type_id.unwrap_or(1) as usize }
}

pub(crate) struct FormCallbacks {
    pub(crate) oninput_name: Callback<InputData>,
    pub(crate) oninput_phone: Callback<InputData>,
    pub(crate) oninput_address: Callback<InputData>,
    pub(crate) onchange_region: Callback<ChangeData>,
    pub(crate) onchange_type: Callback<ChangeData>,
}

/// Universal form for creating/updating representation
pub(crate) fn render_represent_form<T: RepresentFormValues>(
    data: &T,
    callbacks: FormCallbacks,
    regions: &[Region],
    selected_region_id: usize,
    represent_types: &[RepresentationType],
    selected_type_id: usize,
    loading: bool,
) -> Html {
    html!{
        <>
            <div class="mb-4">
                {render_form_input(InputConfig {
                    id: "name",
                    label: get_value_field(&110),
                    value: data.name(),
                    oninput: callbacks.oninput_name,
                    is_disabled: loading,
                    icon: Some("fas fa-building"),
                    add_classes: classes!(""),
                    is_danger: false,
                })}
            </div>
            <div class="columns is-desktop mb-0">
                <div class="column">
                    {render_form_input(InputConfig {
                        id: "tel",
                        label: get_value_field(&56),
                        value: data.phone(),
                        oninput: callbacks.oninput_phone,
                        is_disabled: loading,
                        icon: Some("fas fa-phone"),
                        add_classes: classes!(""),
                        is_danger: false,
                    })}
                </div>
                <div class="column">
                    <div class="field">
                        <label class="label">{get_value_field(&216)}</label>
                        <div class="control">
                            <div class="select is-fullwidth">
                              <select onchange={callbacks.onchange_type} disabled={loading}>
                                { for represent_types.iter().map(|x|
                                    html!{
                                        <option
                                            value={x.representation_type_id.to_string()}
                                            selected={x.representation_type_id == selected_type_id}
                                        >
                                            {&x.representation_type}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="columns is-desktop mb-0">
                <div class="column">
                    <div class="field">
                        <label class="label">{get_value_field(&27)}</label>
                        <div class="control">
                            <div class="select is-fullwidth">
                              <select onchange={callbacks.onchange_region} disabled={loading}>
                                { for regions.iter().map(|x|
                                    html!{
                                        <option
                                            value={x.region_id.to_string()}
                                            selected={x.region_id == selected_region_id}
                                        >
                                            {&x.region}
                                        </option>
                                    }
                                )}
                              </select>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="column">
                    {render_form_input(InputConfig {
                        id: "address",
                        label: get_value_field(&57),
                        value: data.address(),
                        oninput: callbacks.oninput_address,
                        is_disabled: loading,
                        icon: Some("fas fa-map-marker-alt"),
                        add_classes: classes!(""),
                        is_danger: false,
                    })}
                </div>
            </div>
        </>
    }
}