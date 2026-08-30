use yew::{html, Html, classes, Classes, Callback, MouseEvent};
use crate::services::{LocaleKey, unique_id};
use crate::types::Pathname;

use super::ListState;

/// Returns a VNode with Html code of button to download url in the <a> tag
/// (the button is not active if the link is empty)
pub fn ft_download_btn(download_url: String, as_button: bool) -> Html {
    let title = LocaleKey::Download.get_value();
    let class_btn = match as_button {
        true => classes!("button", "is-white"),
        false => classes!("is-white"),
    };
    html!{
        <a class={class_btn}
        href={download_url.clone()}
        disabled={download_url.is_empty()}
        title={title}
        target="_blank">
            <span class="icon">
                <i class="fas fa-file-download" style="color: #1872f0;" aria-hidden="true"></i>
            </span>
            {match as_button {
                true => html!{},
                false => html!{<span>{title}</span>},
            }}
        </a>
    }
}

/// Returns a VNode with Html code of a button with icon and text to download url
/// in the <a> tag (the button is not active if the link is empty)
pub fn ft_download_full_btn(download_url: String) -> Html {
    let title_text = LocaleKey::Download.get_value();
    html!{
        <a class={classes!("button", "is-info", "is-fullwidth")}
        href={download_url.clone()}
        disabled={download_url.is_empty()}
        title={title_text}
        target="_blank">
            <span class="icon">
                <i class="fas fa-file-download" aria-hidden="true"></i>
            </span>
            <span>{title_text}</span>
        </a>
    }
}

/// Returns a VNode with Html code of a button for the "Show more" or "Show less" action
pub fn ft_see_btn(onclick_btn: Callback<MouseEvent>, show_full: bool) -> Html {
    let (title_text, class_icon) = match show_full {
        true => (LocaleKey::SeeLess.get_value(), "fas fa-caret-up"),
        false => (LocaleKey::SeeMore.get_value(), "fas fa-caret-down"),
    };
    html!{
        <button class={classes!("button", "is-white", "is-fullwidth")} onclick={onclick_btn}>
          {title_text}
          <span class="icon" style="color: #1872f0; padding-left: 1rem;">
              <i class={class_icon} aria-hidden="true"></i>
          </span>
        </button>
    }
}

pub(crate) fn ft_change_view_btn(onclick_btn: Callback<MouseEvent>, show_type: &ListState) -> Html {
    html!{
        <button class="button" onclick={onclick_btn}>
            <span class="icon is-small">
                <i class={show_type.get_icon_class()}></i>
            </span>
        </button>
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
            LocaleKey::RemoveFromBookmarks.get_value()
        },
        false => {
            class_fav.push("far");
            LocaleKey::AddToBookmarks.get_value()
        },
    };

    html!{
        <button
        id={unique_id("following-button")}
        class="button"
        onclick={trigger_btn}
        title={title}>
            <span class="icon is-small" style="color: #1872f0;">
                <i class={class_fav}></i>
            </span>
            {match subscribers.is_empty() {
                true => html!{},
                false => html!{<span>{subscribers}</span>},
            }}
        </button>
    }
}

/// Generates a standardized discussion button with a comment icon.
/// The text label automatically hides on mobile viewports for tighter layouts.
pub fn ft_discussion_btn(
    id_btn: &str,
    onclick_action: Callback<MouseEvent>,
    is_active: bool,
) -> Html {
    let active_classes = match is_active {
        true => "button is-info is-light is-active",
        false => "button is-info",
    };
    let title_text = LocaleKey::Discussion.get_value();
    html! {
        <button
            id={unique_id(id_btn)}
            class={active_classes}
            onclick={onclick_action}
            title={title_text}
        >
            <span class="icon is-small">
                <i class="far fa-comments" aria-hidden="true"></i>
            </span>
            <span class="is-hidden-mobile">{title_text}</span>
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
    ft_delete_class_btn(id_btn, trigger_btn, confirm, disabled, classes!("is-fullwidth"))
}

/// Returns a VNode with the Html code of the delete button with confirmation and extensible style classes
pub fn ft_delete_class_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    confirm: bool,
    disabled: bool,
    add_classes: Classes,
) -> Html {
    let mut set_classes = classes!("button", "is-danger", add_classes);
    let title_text = match confirm {
        true => LocaleKey::YesDelete.get_value(),
        false => {
            set_classes.push("is-light");
            LocaleKey::Delete.get_value()
        },
    };
    html!{
        <button
            id={unique_id(id_btn)}
            class={set_classes}
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

/// Returns a pair of buttons (Confirm/Cancel) for delete confirmation
pub fn ft_delete_pair_btn(
    id_btn: &str,
    on_action: Callback<bool>,
    confirm: bool,
    disabled: bool,
    add_classes: Classes,
) -> Html {
    let on_confirm = on_action.reform(|_| true);
    let on_cancel = on_action.reform(|_| false);
    let extended_classes = classes!("is-fullwidth", add_classes.clone());
    if confirm {
    html! {
        <div class="buttons is-fullwidth">
            {ft_cancel_btn(&format!("{}-cancel", id_btn), on_cancel, extended_classes.clone())}
            {ft_delete_class_btn(id_btn, on_confirm, confirm, disabled, extended_classes)}
        </div>
        }
    } else {
        html!{
            ft_delete_class_btn(id_btn, on_confirm, confirm, disabled, extended_classes)
        }
    }
}

/// Returns a VNode with Html code of a small delete button with confirmation
pub fn ft_delete_small_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    confirm: bool,
) -> Html {
    let title_text = match confirm {
        true => LocaleKey::YesDelete.get_value(),
        false => "",
    };

    html!{
        <a id={unique_id(id_btn)} onclick={trigger_btn} title={LocaleKey::Delete.get_value()}>
            <span class="icon" >
                <i class="fa fa-trash" aria-hidden="true" style="color: #f14668;"></i>
            </span>
            <span style="color: #f14668;">{title_text}</span>
        </a>
    }
}

/// Returns a pair of buttons (Cancel/Save) for modal forms
pub fn ft_modal_cancel_save_btn(
    id_btn: &str,
    on_cancel: Callback<MouseEvent>,
    on_save: Callback<MouseEvent>,
    is_fullwidth: bool,
    disabled: bool,
) -> Html {
    let cancel_classes = match is_fullwidth {
        true => classes!("is-fullwidth", "mr-3"),
        false => classes!(""),
    };
    html! {<>
        {ft_cancel_btn(&format!("{}-cancel", id_btn), on_cancel, cancel_classes)}
        {ft_save_btn(&format!("{}-save", id_btn), on_save, is_fullwidth, disabled)}
    </>}
}

/// Returns a VNode with Html code of a save button
pub fn ft_save_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    is_fullwidth: bool,
    disabled: bool,
) -> Html {
    let title_text = LocaleKey::Save.get_value();
    let class_btn = match is_fullwidth {
        true => classes!("button", "is-link", "is-fullwidth"),
        false => classes!("button", "is-link"),
    };
    // if is_loading {
    //     class_btn.push("is-loading");
    // }

    html!{
        <button
            id={unique_id(id_btn)}
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

/// Returns a VNode with Html code are rendered as a submit button
pub fn ft_submit_btn(id_btn: &str) -> Html {
    let title_text = LocaleKey::Save.get_value();

    html!{
        <button
            id={unique_id(id_btn)}
            class={classes!("button", "is-link", "is-fullwidth")}
            type="submit"
            disabled={false}
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
            id={unique_id(id_btn)}
            class={class_btn}
            disabled={disabled}
            onclick={trigger_btn}
            title={title_text.to_string()}>
            <span class="icon">
                <i class="fas fa-plus" aria-hidden="true"></i>
            </span>
            <span class="is-hidden-mobile">{title_text.to_string()}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a create button
pub fn ft_create_btn(
    id_btn: &str,
    class_btn: Classes,
    trigger_btn: Callback<MouseEvent>,
    disabled: bool,
) -> Html {
    let title_text = LocaleKey::Create.get_value();
    let class_btn = classes!("button", "is-fullwidth", "is-success", class_btn);

    html!{
        <button
            id={unique_id(id_btn)}
            class={class_btn}
            disabled={disabled}
            onclick={trigger_btn}
            title={title_text.to_string()}>
            <span>{title_text.to_string()}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a custom button
pub fn ft_custom_btn(
    id_btn: &str,
    title_text: &str,
    class_btn: Classes,
    class_icon: &str,
    trigger_btn: Callback<MouseEvent>,
    disabled: bool,
) -> Html {
    let class_btn = classes!("button", class_btn);
    html!{
        <button
            id={unique_id(id_btn)}
            class={class_btn}
            disabled={disabled}
            onclick={trigger_btn}
            title={title_text.to_string()}>
            <span class="icon">
                <i class={class_icon.to_string()} aria-hidden="true"></i>
            </span>
            <span class="is-hidden-mobile">{title_text.to_string()}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a cancel button
pub fn ft_cancel_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    add_classes: Classes,
) -> Html {
    let title_text = LocaleKey::Cancel.get_value();

    html!{
        <button
            id={unique_id(id_btn)}
            class={classes!("button", "is-warning", add_classes)}
            onclick={trigger_btn}
            title={title_text.to_string()}>
            <span class="icon">
                <i class="fas fa-undo" aria-hidden="true"></i>
            </span>
            <span>{title_text.to_string()}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a return back button
pub fn ft_back_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    title_text: &str,
) -> Html {
    html!{
        <button
            id={unique_id(id_btn)}
            class="button"
            onclick={trigger_btn}
            title={title_text.to_string()}>
            <span class="icon is-small">
                <i class="fas fa-arrow-left" style="color: #1872f0;"></i>
            </span>
            <span>{title_text.to_string()}</span>
        </button>
    }
}

/// Returns a VNode with Html code of a return import button
pub fn ft_import_btn(
    id_btn: &str,
    trigger_btn: Callback<MouseEvent>,
    title_text: &str,
    is_fullwidth: bool,
    disabled: bool,
) -> Html {
    let class_btn = match is_fullwidth {
        true => classes!("button", "is-link", "is-fullwidth"),
        false => classes!("button"),
    };
    html!{
        <button
            id={unique_id(id_btn)}
            class={class_btn}
            disabled={disabled}
            onclick={trigger_btn}
            title={title_text.to_string()}>
            {match is_fullwidth {
                true => html!{
                    <span class="icon is-small">
                        <i class="far fa-save" aria-hidden="true"></i>
                    </span>
                },
                false => html!{
                    <span class="icon is-small">
                        <i class="fas fa-upload" style="color: #1872f0;"></i>
                    </span>
                },
            }}
            <span class="is-hidden-mobile">{LocaleKey::Import.get_value()}</span>
        </button>
    }
}

/// Returns a VNode with the specified URL and text
pub fn simple_link(url: String, label: &str) -> Html {
    html!{<a href={url} target="_blank" rel="noopener noreferrer">{label}</a>}
}

/// Returns a VNode with a styled button with an icon and the title "Settings"
pub fn res_settings_btn(onclick: Callback<MouseEvent>, pathname: Pathname) -> Html {
    let title = LocaleKey::Settings.get_value();
    html!{
      <a class="button" onclick={onclick} href={pathname.get_pathname()} title={title}>
        <span class="icon is-small" >
          <i class={classes!("fa", "fa-tools")}></i>
        </span>
        <span class="is-hidden-mobile">{title}</span>
      </a>
    }
}
