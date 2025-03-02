use yew::{html, Html, InputData, Callback};
use crate::services::get_value_field;

/// Returns a VNode (html code) with an input field to enter a comment on upload files
pub fn commit_msg_field(commit_msg: String, oninput_commit_msg: Callback<InputData>) -> Html {
    let label_commit_msg = get_value_field(&340);
    html!{
        <div class={"column"}>
            <div class={"columns"}>
                <div class={"column is-narrow"}>
                    <p class={"title is-6 select-title"}>{get_value_field(&338)}</p>
                </div>
                <div class={"column"}>
                    <input
                        id={"text-commit-msg"}
                        class={"input is-fullwidth"}
                        type={"text"}
                        maxlength={"225"}
                        placeholder={label_commit_msg}
                        title={label_commit_msg}
                        value={commit_msg}
                        oninput={oninput_commit_msg} />
                    <p class={"help"}>{get_value_field(&339)}</p>
                </div>
            </div>
        </div>
    }
}