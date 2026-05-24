use yew::{classes, html, Callback, Html, InputData, MouseEvent};
use crate::{fragments::buttons::{ft_cancel_btn, ft_delete_class_btn}, services::get_value_field};

/// Generates a card with a delete confirmation modal.
///
/// # Arguments
///
/// * `id_tag` - The string is used to create HTML tag identifiers.
/// * `title` - The title or label for the delete card and button.
/// * `object_name` - The name of the object to be deleted, displayed in the modal.
/// * `confirm_key` - A confirmation string for comparison and confirmation of deletion.
/// * `confirm_text` - A confirmation string inputted by the user to confirm deletion.
/// * `onclick_hide_modal` - Callback invoked to hide or close the modal.
/// * `oninput_delete` - Callback invoked when the user types into the confirmation input field.
/// * `onclick_delete` - Callback invoked when the user confirms deletion.
/// * `hide_delete_modal` - Boolean flag to control whether the modal is visible or hidden.
/// * `disable_delete_btn` - Boolean flag that disables the delete button if the confirmation text and the entered text do not match.
///
/// # Returns
///
/// An `Html` (VNode) node representing the delete card with embedded modal.
pub fn ft_delete_card(
    id_tag: &str,
    title: &str,
    object_name: String,
    confirm_key: String,
    confirm_text: String,
    onclick_hide_modal: Callback<MouseEvent>,
    oninput_delete: Callback<InputData>,
    onclick_delete: Callback<MouseEvent>,
    hide_delete_modal: bool,
    disable_delete_btn: bool,
) -> Html {
    html!{
        <div id={format!("delete-{}-card", id_tag)} class="card">
            <header class="card-header"><p class="card-header-title has-text-danger-dark">{title}</p></header>
            <div class="card-content has-background-danger-light">
                <div class="content">
                        {modal_delete(
                            object_name,
                            confirm_key,
                            confirm_text,
                            onclick_hide_modal.clone(),
                            oninput_delete.clone(),
                            onclick_delete.clone(),
                            hide_delete_modal,
                            disable_delete_btn,
                        )}
                        <p>{get_value_field(&408)}</p>
                        <button
                            id={format!("delete-{}-open-btn", id_tag)}
                            class="button is-danger"
                            onclick={onclick_hide_modal} >
                            {title}
                        </button>
                </div>
            </div>
        </div>
    }
}


/// Creates the modal dialog for delete confirmation, including input validation and action buttons.
fn modal_delete(
    object_name: String,
    confirm_key: String,
    confirm_text: String,
    onclick_hide_modal: Callback<MouseEvent>,
    oninput_delete: Callback<InputData>,
    onclick_delete: Callback<MouseEvent>,
    hide_delete_modal: bool,
    disable_delete_btn: bool,
) -> Html {
    let modal_classes = classes!(
        "modal",
        "is-isolated-modal",
        if hide_delete_modal { None } else { Some("is-active") }
    );

    html! {
        <div class={modal_classes}>
            <div class="modal-background" onclick={onclick_hide_modal.clone()} />
            <div class="modal-content">
                <div id="delete-component-card" class="card p-4">
                    <header class="pb-3">
                        <h2 class="title is-4 has-text-centered">
                            {get_value_field(&409)}
                        </h2>
                    </header>
                    <div id="confirm-danger-modal-content" class="content">
                        <div class="column has-background-danger-light">
                            <span>{get_value_field(&408)}</span>
                            <span class="ml-3">
                                {get_value_field(&410)}
                                <strong>{object_name}</strong>
                                {get_value_field(&411)}
                            </span>
                        </div>
                        <div class="py-2">
                            <p>{get_value_field(&412)}</p>
                        </div>
                        <div class="field">
                            <label class="has-text-weight-bold" for="confirm-name-input">
                                {get_value_field(&413)}
                            </label>
                            <div class="control mt-1">
                                <code class="has-text-black">{confirm_key}</code>
                            </div>
                        </div>
                        <div class="field">
                            <div class="control">
                                <input
                                    id="confirm-name-input"
                                    class="input"
                                    type="text"
                                    value={confirm_text}
                                    oninput={oninput_delete}
                                />
                            </div>
                        </div>
                        <div class="columns mt-4">
                            <div class="column">
                                {ft_cancel_btn("delete-cancel", onclick_hide_modal, classes!("is-fullwidth"))}
                            </div>
                            <div class="column">
                                {ft_delete_class_btn("delete-confirm", onclick_delete, true, disable_delete_btn, classes!("is-fullwidth"))}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}