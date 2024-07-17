use yew::{html, Html, classes, Callback, MouseEvent};
use crate::services::get_value_field;

/// Returns a VNode with Html code of button to download url in the <a> tag
/// (the button is not active if the link is empty)
pub fn ft_download_btn(download_url: String, as_button: bool) -> Html {
    let class_btn = match as_button {
        true => classes!("button", "is-white"),
        false => classes!("is-white"),
    };
    html!{
        <a class={class_btn}
        href={download_url.clone()}
        disabled={download_url.is_empty()}
        title={get_value_field(&126)}
        target={"_blank"}>
            <span class={"icon"}>
                <i class={"fas fa-file-download"} style={"color: #1872f0;"} aria-hidden={"true"}></i>
            </span>
        </a>
    }
}

/// Returns a VNode with Html code of a button with icon and text to download url
/// in the <a> tag (the button is not active if the link is empty)
pub fn ft_download_full_btn(download_url: String) -> Html {
    let title_text = get_value_field(&126);
    html!{
        <a class={classes!("button", "is-info", "is-fullwidth")}
        href={download_url.clone()}
        disabled={download_url.is_empty()}
        title={title_text}
        target={"_blank"}>
            <span class={"icon"}>
                <i class={"fas fa-file-download"} aria-hidden={"true"}></i>
            </span>
            <span>{title_text}</span>
        </a>
    }
}

/// Returns a VNode with Html code of a button for the "Show more" or "Show less" action
pub fn ft_see_btn(show_full_files_btn: Callback<MouseEvent>, show_full_files: bool) -> Html {
    let class_btn = classes!("button", "is-white", "is-fullwidth");
    match show_full_files {
        true => html!{
          <button class={class_btn} onclick={show_full_files_btn}>
            {get_value_field(&99)}
          </button>
        },
        false => html!{
          <button class={class_btn} onclick={show_full_files_btn}>
            {get_value_field(&98)}
          </button>
        },
    }
}

/// Returns a VNode with Html code of a follow button
pub fn ft_follow_btn(
    trigger_btn: Callback<MouseEvent>,
    is_followed: bool,
    subscribers: String,
) -> Html {
    let mut class_fav = vec!["fa-bookmark"];
    let title = match is_followed {
        true => {
            class_fav.push("fas");
            get_value_field(&327)
        },
        false => {
            class_fav.push("far");
            get_value_field(&326)
        },
    };

    html!{
        <button
        id={"following-button"}
        class={"button"}
        onclick={trigger_btn}
        title={title}>
            <span class="icon is-small" style={"color: #1872f0;"}>
                <i class={class_fav}></i>
            </span>
            {match subscribers.is_empty() {
                true => html!{},
                false => html!{<span>{subscribers}</span>},
            }}
        </button>
    }
}

/// Returns a VNode with Html code of a delete button with confirmation
pub fn ft_delete_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    confirm: bool,
    disabled: bool,
) -> Html {
    let title_text = match confirm {
        true => get_value_field(&220),
        false => get_value_field(&135),
    };

    html!{
        <button
            id={id_btn.to_string()}
            class="button is-danger is-fullwidth"
            disabled={disabled}
            onclick={trigger_btn}
            title={title_text}>
            <span class="icon">
                <i class="fa fa-trash" aria-hidden="true"></i>
            </span>
            <span>{title_text}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a save button
pub fn ft_save_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    is_fullwidth: bool,
    disabled: bool,
) -> Html {
    let title_text = get_value_field(&46);
    let class_btn = match is_fullwidth {
        true => classes!("button", "is-link", "is-fullwidth"),
        false => classes!("button", "is-link"),
    };
    // if is_loading {
    //     class_btn.push("is-loading");
    // }

    html!{
        <button
            id={id_btn.to_string()}
            class={class_btn}
            disabled={disabled}
            onclick={trigger_btn}
            title={title_text}>
            <span class="icon">
                <i class="far fa-save" aria-hidden="true"></i>
            </span>
            <span>{title_text}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a add button
pub fn ft_add_btn(
    id_btn: &str,
    title_text: &str,
    trigger_btn: Callback<MouseEvent>,
    is_fullwidth: bool,
    disabled: bool,
) -> Html {
    let class_btn = match is_fullwidth {
        true => classes!("button", "is-success", "is-fullwidth"),
        false => classes!("button", "is-success"),
    };

    html!{
        <button
            id={id_btn.to_string()}
            class={class_btn}
            disabled={disabled}
            onclick={trigger_btn}
            title={title_text.to_string()}>
            <span class="icon">
                <i class="fas fa-plus" aria-hidden="true"></i>
            </span>
            <span>{title_text.to_string()}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a cancel button
pub fn ft_cancel_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
) -> Html {
    let title_text = get_value_field(&221);

    html!{
        <button
            id={id_btn.to_string()}
            class={classes!("button", "is-warning", "is-fullwidth")}
            onclick={trigger_btn}
            title={title_text.to_string()}>
            <span class="icon">
                <i class="fas fa-undo" aria-hidden="true"></i>
            </span>
            <span>{title_text.to_string()}</span>
        </button>
    }
}