use chrono::format::strftime::StrftimeItems;
use chrono::NaiveDateTime;
use yew::{html, Html};
use crate::services::{get_lang, get_value_field};

/// Returns Html code with two dates in abbr tag
pub(crate) fn two_dates_display(created_dt: &NaiveDateTime, updated_dt: &NaiveDateTime) -> Html {
    let lang = get_lang().unwrap_or(String::new());
    let (fmt_dt, fmt_d) = match lang.as_str() {
        "ru" => (StrftimeItems::new("%d.%m.%Y %H:%M:%S"), StrftimeItems::new("%d.%m.%Y")),
        _ => (StrftimeItems::new("%Y-%m-%d %H:%M:%S"), StrftimeItems::new("%Y-%m-%d")),
    };

    html!{
        <abbr title = {format!(
            "{} {}\n{} {}",
            get_value_field(&276),
            created_dt.format_with_items(fmt_dt.clone()),
            get_value_field(&277),
            updated_dt.format_with_items(fmt_dt)
        )}>
            {format!(
                "{} {}",
                get_value_field(&277),
                updated_dt.format_with_items(fmt_d)
            )}
        </abbr>
    }
}

/// Returns VNode (Html) with date, with date and time information in addr tag
pub(crate) fn date_display(dt: &NaiveDateTime) -> Html {
    let lang = get_lang().unwrap_or(String::new());
    let (fmt_dt, fmt_d) = match lang.as_str() {
        "ru" => (StrftimeItems::new("%d.%m.%Y %H:%M:%S"), StrftimeItems::new("%d.%m.%Y")),
        _ => (StrftimeItems::new("%Y-%m-%d %H:%M:%S"), StrftimeItems::new("%Y-%m-%d")),
    };

    html!{
        <abbr title = {dt.format_with_items(fmt_dt).to_string()}>
            {dt.format_with_items(fmt_d).to_string()}
        </abbr>
    }
}