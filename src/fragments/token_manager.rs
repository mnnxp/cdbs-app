use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::{debug, warn};

use crate::services::{get_single_from_value, get_token, get_value_response, resp_parsing, set_logged_user, set_token};
use crate::types::UserToken;
use crate::gqls::make_query;
use crate::gqls::user::{
    CheckToken, check_token,
    GetToken, get_token
};

pub(crate) enum TokenStatus {
    Checking,
    Valid,
    Expired,
    Refreshing,
    Refreshed,
}

pub(crate) struct TokenManager {
    props: Props,
    link: ComponentLink<Self>,
    status: TokenStatus,
}

#[derive(Properties, Clone)]
pub(crate) struct Props {
    pub(crate) on_expired: Option<Callback<()>>,
}

pub(crate) enum Msg {
    CheckToken,
    CheckTokenResult(String),
    NewToken,
    NewTokenResult(String),
    TokenExpired,
}

impl Component for TokenManager {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            status: TokenStatus::Checking,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::CheckToken);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let link = self.link.clone();
        match msg {
            Msg::CheckToken => {
                if get_token().is_none() {
                    link.send_message(Msg::TokenExpired);
                    return true;
                }
                self.status = TokenStatus::Refreshing;
                spawn_local(async move {
                    let res = make_query(CheckToken::build_query(
                        check_token::Variables)
                    ).await.unwrap();
                    link.send_message(Msg::CheckTokenResult(res));
                });
            }
            Msg::CheckTokenResult(res) => {
                match get_value_response(res) {
                    Ok(value) => {
                        let is_valid = get_single_from_value(&value, "isTokenValid").unwrap_or(false);
                        let days = get_single_from_value(&value, "tokenDaysUntilExpiry").unwrap_or(0);
                        match is_valid {
                            false => link.send_message(Msg::TokenExpired),
                            true if days <= 3 => {
                                link.send_message(Msg::NewToken);
                                self.status = TokenStatus::Refreshing;
                            },
                            true => self.status = TokenStatus::Valid,
                        }
                    }
                    Err(err) => debug!("Failed to check token: {:?}", err),
                }
            }
            Msg::NewToken => {
                spawn_local(async move {
                    let res = make_query(GetToken::build_query(
                        get_token::Variables)
                    ).await.unwrap();
                    link.send_message(Msg::NewTokenResult(res));
                });
            }
            Msg::NewTokenResult(res) => {
                match resp_parsing::<UserToken>(res, "getToken") {
                    Ok(user_token) => {
                        debug!("Token refreshed successfully");
                        set_token(Some(user_token.to_string()));
                        self.status = TokenStatus::Refreshed;
                    },
                    Err(err) => debug!("Failed to refresh token: {:?}", err),
                }
            }
            Msg::TokenExpired => {
                self.status = TokenStatus::Expired;
                set_token(None);
                set_logged_user(None);
                if let Some(on_expired) = &self.props.on_expired {
                    on_expired.emit(());
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.status {
            TokenStatus::Checking => debug!("Checking session..."),
            TokenStatus::Valid => debug!("Token valid"),
            TokenStatus::Expired => warn!("Session expired. Redirecting to login..."),
            TokenStatus::Refreshing => debug!("Refreshing token..."),
            TokenStatus::Refreshed => debug!("Token refreshed"),
        }
        html!{}
    }
}