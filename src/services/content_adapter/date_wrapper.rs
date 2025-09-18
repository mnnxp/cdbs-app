use chrono::format::strftime::StrftimeItems;
use chrono::NaiveDateTime;
use yew::{html, Html};
use crate::services::get_lang;

/// Contains three time formats
/// to display in different attributes of the time tag
struct StrFmtDate<'a> {
    title: StrftimeItems<'a>,
    datetime: StrftimeItems<'a>,
    text: StrftimeItems<'a>,
}

/// Returns VNode (Html) with date, with date and time information in time tag
pub(crate) fn date_display(dt: &NaiveDateTime) -> Html {
    let srtfmt = srtfmt_dt();
    let title = dt.format_with_items(srtfmt.title).to_string();
    html!{
        <time datetime={dt.format_with_items(srtfmt.datetime).to_string()} title={title}>
            <span>{dt.format_with_items(srtfmt.text).to_string()}</span>
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