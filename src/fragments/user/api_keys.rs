use yew::{html, classes, Component, ComponentLink, Html, Properties, ShouldRender, InputData};
use chrono::{NaiveDate, DateTime, Utc, Duration, TimeZone};
use wasm_bindgen_futures::spawn_local;
use graphql_client::GraphQLQuery;

use crate::fragments::copy_button::CopyButton;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::buttons::{ft_delete_pair_btn, ft_add_btn, ft_custom_btn};
use crate::fragments::modal::ModalBlock;
use crate::fragments::notification::show_notification;
use crate::error::Error;
use crate::services::content_adapter::DateDisplay;
use crate::services::{resp_parsing, unique_id, LocaleKey};
use crate::types::ApiKeyData;
use crate::gqls::make_query;
use crate::gqls::api_keys::{
    GetApiKeys, get_api_keys,
    CreateApiKey, create_api_key,
    DeleteApiKey, delete_api_key,
    RotateApiKey, rotate_api_key,
};

pub(crate) enum Msg {
    FetchApiKeys,
    ApiKeysLoaded(String),
    ShowCreateModal,
    HideCreateModal,
    UpdateKeyName(String),
    UpdateExpiry(String),
    CreateKey,
    KeyCreated(String),
    ConfirmDelete(i64, bool),
    DeleteKey(i64),
    KeyDeleted(String),
    RotateKey(i64),
    KeyRotated(String),
    HideNewKeyModal,
    ResponseError(Error),
    ClearError,
}

pub(crate) struct ApiKeyManager {
    link: ComponentLink<Self>,
    api_keys: Vec<ApiKeyData>,
    show_create_modal: bool,
    show_new_key_modal: bool,
    new_key_name: String,
    new_key_expiry: Option<DateTime<Utc>>,
    new_key_token: String,
    delete_confirm_id: i64,
    loading: bool,
    error: Option<Error>,
    notification: Option<String>,
    token_input_id: String,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {}

impl Component for ApiKeyManager {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            // props,
            api_keys: Vec::new(),
            show_create_modal: false,
            show_new_key_modal: false,
            new_key_name: String::new(),
            new_key_expiry: Some(Utc::now() + Duration::days(365)),
            new_key_token: String::new(),
            delete_confirm_id: 0,
            loading: false,
            error: None,
            notification: None,
            token_input_id: unique_id("api-key-token"),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::FetchApiKeys);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();

        match msg {
            Msg::FetchApiKeys => {
                self.loading = true;
                spawn_local(async move {
                    let res = make_query(GetApiKeys::build_query(
                        get_api_keys::Variables
                    )).await.unwrap();
                    link.send_message(Msg::ApiKeysLoaded(res));
                });
            }
            Msg::ApiKeysLoaded(res) => {
                self.loading = false;
                match resp_parsing::<Vec<ApiKeyData>>(res, "apiKeys") {
                    Ok(keys) => {
                        self.api_keys = keys;
                        // Sort: active first
                        self.api_keys.sort_by(|a, b| b.is_active.cmp(&a.is_active));
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }

            Msg::ShowCreateModal => {
                self.show_create_modal = true;
                self.new_key_name = String::new();
                self.new_key_expiry = Some(Utc::now() + Duration::days(365));
                self.error = None;
            }
            Msg::HideCreateModal => {
                self.show_create_modal = false;
                self.new_key_name = String::new();
            }
            Msg::UpdateKeyName(name) => self.new_key_name = name,
            Msg::UpdateExpiry(value) => {
                if value.is_empty() {
                    self.new_key_expiry = None;
                }
                else if let Ok(naive_date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                    if let Some(naive_datetime) = naive_date.and_hms_opt(0, 0, 0) {
                        self.new_key_expiry = Some(Utc.from_utc_datetime(&naive_datetime));
                    }
                }
            },
            Msg::CreateKey => {
                if self.new_key_name.is_empty() || self.loading {
                    return true;
                }
                self.loading = true;
                let name = self.new_key_name.clone();
                let expires_at = self.new_key_expiry;
                spawn_local(async move {
                    let res = make_query(CreateApiKey::build_query(
                        create_api_key::Variables {
                            name,
                            expires_at,
                        }
                    )).await.unwrap();
                    link.send_message(Msg::KeyCreated(res));
                });
            }
            Msg::KeyCreated(res) => {
                self.loading = false;
                match resp_parsing(res, "createApiKey") {
                    Ok(value) => {
                        self.new_key_token = value;
                        self.show_create_modal = false;
                        self.show_new_key_modal = true;
                        link.send_message(Msg::FetchApiKeys);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::HideNewKeyModal => {
                self.show_new_key_modal = false;
                self.new_key_token.clear();
            }
            Msg::ConfirmDelete(key_id, confirm) => {
                match self.delete_confirm_id == key_id {
                    true if confirm => {
                        link.send_message(Msg::DeleteKey(key_id));
                        self.delete_confirm_id = 0;
                    }
                    true => self.delete_confirm_id = 0,
                    false => self.delete_confirm_id = key_id,
                }
            }
            Msg::DeleteKey(key_id) => {
                self.loading = true;
                spawn_local(async move {
                    let res = make_query(DeleteApiKey::build_query(
                        delete_api_key::Variables { key_id }
                    )).await.unwrap();
                    link.send_message(Msg::KeyDeleted(res));
                });
            }
            Msg::KeyDeleted(res) => {
                self.loading = false;
                match resp_parsing(res, "deleteApiKey") {
                    Ok(result) => {
                        self.delete_confirm_id = 0;
                        if result {
                            self.notification = Some(LocaleKey::KeyDeleted.get_value().to_string());
                        }
                        link.send_message(Msg::FetchApiKeys);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::RotateKey(key_id) => {
                self.loading = true;
                spawn_local(async move {
                    let res = make_query(RotateApiKey::build_query(
                        rotate_api_key::Variables { key_id }
                    )).await.unwrap();
                    link.send_message(Msg::KeyRotated(res));
                });
            }
            Msg::KeyRotated(res) => {
                self.loading = false;
                match resp_parsing(res, "rotateApiKey") {
                    Ok(token) => {
                        self.new_key_token = token;
                        self.show_new_key_modal = true;
                        link.send_message(Msg::FetchApiKeys);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            }
            Msg::ResponseError(err) => {
                self.error = Some(err);
                self.loading = false;
            }
            Msg::ClearError => {
                self.error = None;
                self.notification = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);

        html! {
            <div id="api-keys" class="card api-key-manager">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()} />
                {show_notification(
                    self.notification.as_deref().unwrap_or_default(),
                    "is-success",
                    self.notification.is_some()
                )}
                <header class="card-header">
                    <div class="card-header-title">
                        <p class="is-size-5 has-text-weight-semibold">
                            {LocaleKey::ApiKeys.get_value()}
                        </p>
                        <div class="buttons right-side">
                            {ft_add_btn(
                                "create-api-key",
                                LocaleKey::CreateNewApiKey.get_value(),
                                self.link.callback(|_| Msg::ShowCreateModal),
                                false,
                                self.loading,
                            )}
                        </div>
                    </div>
                </header>
                <div class="card-content">
                    <div class="content">
                        // Keys table
                        {self.render_table()}
                    </div>
                </div>
                // Create key modal window
                {self.render_create_modal()}
                // Show new key modal window
                {self.render_new_key_modal()}
            </div>
        }
    }
}

impl ApiKeyManager {
    fn render_table(&self) -> Html {
        if self.api_keys.is_empty() && !self.loading {
            return html! {
                <div class="notification is-info is-light">
                    <span class="icon"><i class="fas fa-info-circle"></i></span>
                    {LocaleKey::NoApiKeys.get_value()}
                </div>
            };
        }
        html! {
            <div class="table-container">
                <table class="table is-fullwidth is-striped is-hoverable">
                    <thead>
                        <tr>
                            <th>{LocaleKey::Name.get_value()}</th>
                            <th>{LocaleKey::Status.get_value()}</th>
                            <th>{LocaleKey::LastUsed.get_value()}</th>
                            <th>{LocaleKey::ExpiresAt.get_value()}</th>
                            <th>{LocaleKey::CreatedAtLabel.get_value()}</th>
                            <th>{LocaleKey::Action.get_value()}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for self.api_keys.iter().map(|key| self.render_key_row(key))}
                    </tbody>
                </table>
            </div>
        }
    }

    fn render_key_row(&self, key: &ApiKeyData) -> Html {
        let key_id = key.id;
        let is_active = key.is_active;
        // Determine the status color
        let status_class = if is_active { "is-success" } else { "is-danger" };
        let status_text = if is_active {
            LocaleKey::Active.get_value()
        } else {
            LocaleKey::Inactive.get_value()
        };
        // Check if the key is about to expire (less than 7 days)
        let now = Utc::now();
        let expires_soon = key.expires_at.signed_duration_since(now).num_days() < 7 && is_active;
        let on_delete_action = self.link.callback(move |confirm| Msg::ConfirmDelete(key_id, confirm));

        html! {
            <tr class={if !is_active { "has-background-white-ter" } else { "" }}>
                <td>
                    <span class="has-text-weight-medium">{&key.name}</span>
                </td>
                <td>
                    <span class={classes!("tag", status_class)}>{status_text}</span>
                    {if expires_soon {
                        html! { <span class="tag is-warning is-light">{LocaleKey::ExpiresSoon.get_value()}</span> }
                    } else {
                        html! {}
                    }}
                </td>
                <td>
                    {match &key.last_used_at {
                        Some(date) => html! { {date.date_to_display()} },
                        None => html! { <span class="has-text-grey-light">{LocaleKey::NeverUsed.get_value()}</span> },
                    }}
                </td>
                <td>
                    {key.expires_at.date_to_display()}
                </td>
                <td>
                    {key.created_at.date_to_display()}
                </td>
                <td>
                    <div class="buttons are-small">
                        {if is_active {
                            {ft_custom_btn(
                                &format!("rotate-api-key-{}", key_id),
                                LocaleKey::Rotate.get_value(),
                                classes!("is-warning", "is-light", "is-small"),
                                "fas fa-sync-alt",
                                self.link.callback(move |_| Msg::RotateKey(key_id)),
                                self.loading,
                            )}
                        } else {
                            html! {}
                        }}
                        {ft_delete_pair_btn(
                            &format!("delete-api-key-{}", key.id),
                            on_delete_action,
                            self.delete_confirm_id == key.id,
                            self.loading,
                            classes!("is-half")
                        )}
                    </div>
                </td>
            </tr>
        }
    }

    fn render_create_modal(&self) -> Html {
        let is_disabled = self.new_key_name.is_empty() || self.loading;
        let expiry_str = self.new_key_expiry
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_default();
        let set_cutom_date = self.link.callback(|ev: InputData| {
            let date_str = ev.value;
            Msg::UpdateExpiry(date_str)
        });
        let input_name_id = unique_id("input-name-key");
        let input_expiry_id = unique_id("input-expiry-key");

        html! {
            <ModalBlock
                modal_id="create-api-key"
                title={LocaleKey::CreateNewApiKey.get_value()}
                is_active={self.show_create_modal}
                on_close={self.link.callback(|_| Msg::HideCreateModal)}
                on_save={Some(self.link.callback(|_| Msg::CreateKey))}
                save_disabled={is_disabled}
            >
                <div class="content">
                    <div class="field">
                        <label for={input_name_id.clone()} class="label">{LocaleKey::Name.get_value()}</label>
                        <div class="control">
                            <input
                                id={input_name_id}
                                class="input"
                                type="text"
                                placeholder={LocaleKey::EnterKeyName.get_value()}
                                value={self.new_key_name.clone()}
                                oninput={self.link.callback(|ev: InputData| Msg::UpdateKeyName(ev.value))}
                            />
                        </div>
                        <p class="help">{LocaleKey::KeyNameHelp.get_value()}</p>
                    </div>
                    <div class="columns">
                        <div class="column is-one-third">
                            <div class="field">
                                <label for={input_expiry_id.clone()} class="label">{LocaleKey::ExpiresAt.get_value()}</label>
                                <div class="control">
                                    <input
                                        id={input_expiry_id}
                                        class="input"
                                        type="date"
                                        value={expiry_str}
                                        oninput={set_cutom_date}
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </ModalBlock>
        }
    }

    fn render_new_key_modal(&self) -> Html {
        html! {
            <ModalBlock
                modal_id="new-api-key"
                title={LocaleKey::NewApiKey.get_value()}
                is_active={self.show_new_key_modal}
                on_close={self.link.callback(|_| Msg::HideNewKeyModal)}
                on_save={None}
                save_disabled={false}
            >
                <div class="content">
                    <div class="notification is-warning is-light">
                        <span class="icon"><i class="fas fa-exclamation-triangle"></i></span>
                        {LocaleKey::CopyKeyWarning.get_value()}
                    </div>
                    <div class="field has-addons">
                        <div class="control is-expanded">
                            <input
                                id={self.token_input_id.clone()}
                                class="input is-link"
                                type="text"
                                value={self.new_key_token.clone()}
                                readonly=true
                            />
                        </div>
                        <div class="control">
                            <CopyButton
                                text={self.new_key_token.clone()}
                                show_text=true
                                reset_delay={3000}
                            />
                        </div>
                    </div>
                    {ft_custom_btn(
                        "got-it",
                        LocaleKey::GotIt.get_value(),
                        classes!("is-primary", "is-fullwidth", "mt-4"),
                        "",
                        self.link.callback(|_| Msg::HideNewKeyModal),
                        false,
                    )}
                </div>
            </ModalBlock>
        }
    }
}