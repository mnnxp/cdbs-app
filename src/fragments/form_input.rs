use yew::{classes, html, Callback, Classes, Html, InputData};

use crate::services::{get_value_field, unique_id};

/// Configuration for custom input fields
pub(crate) struct InputConfig {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) value: String,
    pub(crate) oninput: Callback<InputData>,
    pub(crate) is_disabled: bool,
    pub(crate) icon: Option<&'static str>,
    pub(crate) add_classes: Classes,
    pub(crate) is_danger: bool,
}

impl InputConfig {
    /// Creates a profile form input field with standardized styling for user profiles.
    ///
    /// # Arguments
    /// * `id` - HTML element ID and field identifier (e.g., "username", "email")
    /// * `label_id` - Localization key for the field label
    /// * `value` - Optional current value of the field
    /// * `oninput` - Callback triggered when input value changes
    /// * `icon` - Optional FontAwesome icon class (e.g., "fas fa-user")
    ///
    /// # Returns
    /// Fully configured input field with consistent profile form styling
    ///
    /// # Example
    /// ```rust
    /// InputConfig::profile_input("username", &50, self.user.username.clone(), oninput_username, Some("fas fa-user"))
    /// ```
    pub(crate) fn profile_input(
        id: &'static str,
        label_id: &usize,
        value: Option<String>,
        oninput: Callback<InputData>,
        icon: Option<&'static str>,
        loading: bool,
        is_danger: bool,
    ) -> Html {
        render_form_input(InputConfig {
            id,
            label: get_value_field(label_id),
            value: value.unwrap_or_default(),
            oninput,
            is_disabled: loading,
            icon,
            add_classes: classes!(""),
            is_danger,
        })
    }

    /// Creates a company form input field with automatic icon selection based on field type.
    ///
    /// # Arguments
    /// * `id` - HTML element ID and field identifier (supports: "inn", "email", "tel", "address", "site_url")
    /// * `label_id` - Localization key for the field label
    /// * `value` - Optional reference to current field value
    /// * `oninput` - Callback triggered when input value changes
    ///
    /// # Returns
    /// Fully configured input field with appropriate icon for the field type
    ///
    /// # Icon Mapping
    /// | Field ID  | Icon Class              |
    /// |-----------|-------------------------|
    /// | "inn"     | "fas fa-file-invoice"   |
    /// | "email"   | "fas fa-envelope"       |
    /// | "tel"     | "fas fa-phone"          |
    /// | "address" | "fas fa-map-marker-alt" |
    /// | "site_url"| "fas fa-globe"          |
    /// | others    | None                    |
    ///
    /// # Example
    /// ```rust
    /// InputConfig::company_input("inn", &163, self.company.inn.as_ref(), oninput_inn)
    /// ```
    pub(crate) fn company_input(
        id: &'static str,
        label_id: &usize,
        value: Option<&String>,
        oninput: Callback<InputData>,
        loading: bool,
    ) -> Html {
        let icon = match id {
            // "orgname" | "shortname" => Some("fas fa-building"),
            "inn" => Some("fas fa-file-invoice"),
            "email" => Some("fas fa-envelope"),
            "tel" => Some("fas fa-phone"),
            "address" => Some("fas fa-map-marker-alt"),
            "site_url" => Some("fas fa-globe"),
            _ => None,
        };
        render_form_input(InputConfig {
            id,
            label: get_value_field(label_id),
            value: value.cloned().unwrap_or_default(),
            oninput,
            is_disabled: loading,
            icon,
            add_classes: classes!(""),
            is_danger: false,
        })
    }
}

/// Global generator of input fields and text areas
pub(crate) fn render_form_input(config: InputConfig) -> Html {
    let field_id = unique_id(&config.id);
    let (input_tag, input_type, base_class) = match config.id {
        "email" => ("input", "email", "input"),
        "description" => ("textarea", "text", "textarea"),
        "password" => ("input", "password", "input"),
        "date" => ("input", "date", "input"),
        "tel" => ("input", "tel", "input"),
        _ => ("input", "text", "input"),
    };
    let input_classes = classes!(
        base_class,
        config.add_classes,
        if config.is_danger { Some("is-danger") } else { None }
    );

    let control_class = classes!(
        "control",
        if config.icon.is_some() && input_tag != "textarea" { Some("has-icons-left") } else { None },
        if config.is_danger && input_tag != "textarea" { Some("has-icons-right") } else { None }
    );

    html! {
        <div class={"field"}>
            <label class={"label"} for={field_id.clone()}>{config.label}</label>
            <div class={control_class}>
                <@{input_tag}
                    id={field_id}
                    class={input_classes}
                    type={input_type}
                    placeholder={config.label}
                    value={config.value}
                    oninput={config.oninput}
                    disabled={config.is_disabled}
                />
                {match config.icon {
                    Some(icon_class) if input_tag != "textarea" => {
                        html! {
                            <span class={"icon is-small is-left"}>
                                <i class={icon_class}></i>
                            </span>
                        }
                    }
                    _ => html! {}
                }}
                {if config.is_danger && input_tag != "textarea" {
                    html! {
                        <span class="icon is-small is-right has-text-danger">
                            <i class="fas fa-exclamation-triangle"></i>
                        </span>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
