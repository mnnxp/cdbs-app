use yew::{html, Html, Callback, InputData, MouseEvent};
use crate::services::get_value_field;

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
    let class_modal = match hide_delete_modal {
        true => "modal",
        false => "modal is-active",
    };

    html!{
        <div class={class_modal}>
            <div class="modal-background" onclick={onclick_hide_modal.clone()} />
            <div class="modal-content">
                <div id="delete-component-card" class="card">
                    <header class="card-header">
                        <h2 class="card-header-title">
                            {get_value_field(&409)}
                        </h2>
                    </header>
                    <span tabindex="0"></span>
                    <div class="card-content">
                        <div class="content">
                            <div id="confirm-danger-modal-content">
                                <div class="column has-background-danger-light">
                                    <span>{get_value_field(&408)}</span>
                                    <span>
                                        {" "}
                                        {get_value_field(&410)}
                                        <strong>{object_name}</strong>
                                        {get_value_field(&411)}
                                    </span>
                                </div>
                                <div class="pb-1 mb-1 pt-1 mt-1">
                                    <p>{get_value_field(&412)}</p>
                                </div>
                                <div class="pb-1 mb-1">
                                    <label class="has-text-weight-bold" for="confirm-name-input">{get_value_field(&413)}</label>
                                    <br/>
                                    <code class="has-text-black">{confirm_key}</code>
                                </div>
                                <fieldset id="confirm-fieldset" class="field" aria-invalid="true">
                                    <legend tabindex="-1"> </legend>
                                    <input
                                        id="confirm-name-input"
                                        class="input"
                                        type="text"
                                        value={confirm_text.clone()}
                                        oninput={oninput_delete}
                                        />
                                </fieldset>
                                <div class="column">
                                    <div class="buttons">
                                        <button id="cancel-btn" class="button is-warning is-half" onclick={onclick_hide_modal}>
                                            {get_value_field(&221)} // Cancel
                                        </button>
                                        <button
                                        id="confirm-btn"
                                        class="button is-danger is-half"
                                        disabled={disable_delete_btn}
                                        onclick={onclick_delete} >
                                            {get_value_field(&220)} // Yes, delete
                                        </button>
                                    </div>
                                </div>
                            </div>
                            <span tabindex="0"></span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}