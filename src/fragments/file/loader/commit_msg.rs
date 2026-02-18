use yew::{html, Html, InputData, Callback};
use crate::{services::get_value_field, types::UUID};

/// Returns a VNode (html code) with an input field to enter a comment on upload files
pub fn commit_msg_field(object_uuid: UUID, commit_msg: String, oninput_commit_msg: Callback<InputData>) -> Html {
    let label_commit_msg = get_value_field(&340);
    html!{
        <div class={"column"}>
            <div class={"field mb-5"}>
                <label class={"label"} for={"text-commit-msg"}>{get_value_field(&338)}</label>
                <div class={"control"}>
                    <input
                        id={format!("text-commit-msg-{}", object_uuid)}
                        class={"input is-fullwidth"}
                        type={"text"}
                        maxlength={"225"}
                        placeholder={label_commit_msg}
                        title={label_commit_msg}
                        value={commit_msg}
                        oninput={oninput_commit_msg} />
                </div>
                <p class={"help"}>{get_value_field(&339)}</p>
            </div>
        </div>
    }
}