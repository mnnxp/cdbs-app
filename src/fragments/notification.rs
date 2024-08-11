use yew::{html, Html, classes};

/// Returns a notification with the specified text and color (style)
pub fn show_notification(text: &str, style: &str, is_show: bool) -> Html {
    let class_notif = classes!("notification", "custom-notif", "hide-after", style.to_string());

    match is_show {
        true => html!{
            <div class={class_notif}>
                {text} // Data updated
            </div>
        },
        false => html!{},
    }
}