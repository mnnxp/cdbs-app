use yew::{html, Html, classes};

/// Returns a notification with the specified text and color (style)
/// Disappears automatically after 3 seconds with progress bar
pub fn show_notification(text: &str, style: &str, is_show: bool) -> Html {
    if !is_show {
        return html!{}
    }
    html!{
        <div class={classes!("notification", "p-3", "custom-notif", "hide-after", style.to_string())}>
            {text}
        </div>
    }
}