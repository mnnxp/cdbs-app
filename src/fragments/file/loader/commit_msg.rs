use yew::{html, Html, InputData, Callback};
use crate::{services::LocaleKey, types::UUID};

/// Returns a VNode (html code) with an input field to enter a comment on upload files
pub(crate) fn commit_msg_field(object_uuid: UUID, commit_msg: String, oninput_commit_msg: Callback<InputData>) -> Html {
    let label_commit_msg = LocaleKey::MaxMessageLength.get_value();
    let tag_id = format!("text-commit-msg-{}", object_uuid);
    html!{
        <div class={"column"}>
            <div class={"field mb-5"}>
                <label class={"label"} for={tag_id.clone()}>{LocaleKey::MessageToChange.get_value()}</label>
                <div class={"control"}>
                    <input
                        id={tag_id}
                        class={"input is-fullwidth"}
                        type={"text"}
                        maxlength={"500"}
                        placeholder={label_commit_msg}
                        title={label_commit_msg}
                        value={commit_msg}
                        oninput={oninput_commit_msg} />
                </div>
                <p class={"help"}>{LocaleKey::MessageHelp.get_value()}</p>
            </div>
        </div>
    }
}