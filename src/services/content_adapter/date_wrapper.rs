use chrono::format::strftime::StrftimeItems;
use chrono::NaiveDateTime;
use yew::{html, Html};
use crate::services::{get_lang, get_value_field};

/// Contains three time formats
/// to display in different attributes of the time tag
struct StrFmtDate<'a> {
    title: StrftimeItems<'a>,
    datetime: StrftimeItems<'a>,
    text: StrftimeItems<'a>,
}

/// Returns Html code with two dates in abbr tag
pub(crate) fn two_dates_display(created_dt: &NaiveDateTime, updated_dt: &NaiveDateTime) -> Html {
    let srtfmt = srtfmt_dt();
    html!{
        <time
        title={format!(
            "{} {}\n{} {}",
            get_value_field(&276),
            created_dt.format_with_items(srtfmt.title.clone()),
            get_value_field(&277),
            updated_dt.format_with_items(srtfmt.title)
        )}
        datetime={updated_dt.format_with_items(srtfmt.datetime).to_string()}>
            {format!(
                "{} {}",
                get_value_field(&277),
                updated_dt.format_with_items(srtfmt.text)
            )}
        </time>
    }
}

/// Returns VNode (Html) with date, with date and time information in time tag
pub(crate) fn date_display(dt: &NaiveDateTime) -> Html {
    let srtfmt = srtfmt_dt();
    html!{
        <time
        title={dt.format_with_items(srtfmt.title).to_string()}
        datetime={dt.format_with_items(srtfmt.datetime).to_string()}>
            {dt.format_with_items(srtfmt.text).to_string()}
        </time>
    }
}

/// Sets the time format depending on the language
fn srtfmt_dt<'a>() -> StrFmtDate<'a> {
    let datetime = StrftimeItems::new("%Y-%m-%d");
    let (title, text) = match get_lang().unwrap_or(String::new()).as_str() {
        "ru" => (StrftimeItems::new("%d.%m.%Y %H:%M:%S"), StrftimeItems::new("%d.%m.%Y")),
        _ => (StrftimeItems::new("%Y-%m-%d %H:%M:%S"), datetime.clone()),
    };
    StrFmtDate{
        title,
        datetime,
        text
    }
}