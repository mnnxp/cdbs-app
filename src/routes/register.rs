use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::{set_token, Auth};
use crate::types::{RegisterInfo, RegisterInfoWrapper, SlimUser, SlimUserWrapper};

/// Register page
pub struct Register {
    auth: Auth,
    error: Option<Error>,
    props: Props,
    request: RegisterInfo,
    response: Callback<Result<SlimUserWrapper, Error>>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// Callback when user is registered in successfully
    pub callback: Callback<SlimUser>,
}

pub enum Msg {
    Request,
    Response(Result<SlimUserWrapper, Error>),
    Ignore,
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateUsername(String),
}

impl Component for Register {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Register {
            auth: Auth::new(),
            error: None,
            request: RegisterInfo::default(),
            response: link.callback(Msg::Response),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let request = RegisterInfoWrapper {
                    user: self.request.clone(),
                };
                self.task = Some(self.auth.register(request, self.response.clone()));
            }
            Msg::Response(Ok(user_info)) => {
                // set_token(Some(user_info.user.token.clone()));
                self.props.callback.emit(user_info.user);
                self.error = None;
                self.task = None;
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateEmail(email) => {
                self.request.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = password;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
        });
        let oninput_username = self
            .link
            .callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));

        html! {
            <div class="auth-page">
                <h1 class="title">{ "Sign Up" }</h1>
                <h2 class="subtitle">
                    <RouterAnchor<AppRoute> route=AppRoute::Login>
                        { "Have an account?" }
                    </RouterAnchor<AppRoute>>
                </h2>
                <ListErrors error=&self.error />
                <form onsubmit=onsubmit>
                    <fieldset>
                        <fieldset class="field">
                            <label class="label">{"Firstname"}</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Text input"
                                    // value=&self.request.firstname
                                    // oninput=oninput_firstname
                                    />
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Lastname"}</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Text input"
                                    // value=&self.request.lastname
                                    // oninput=oninput_lastname
                                    />
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Secondname"}</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Text input"
                                    // value=&self.request.secondname
                                    // oninput=oninput_secondname
                                    />
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Username"}</label>
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    class="input"
                                    type="text"
                                    placeholder="Username"
                                    value=&self.request.username
                                    oninput=oninput_username
                                    />
                                <span class="icon is-small is-left">
                                    <i class="fas fa-user"></i>
                                </span>
                                <span class="icon is-small is-right">
                                <i class="fas fa-check"></i>
                                </span>
                            </div>
                            // <p class="help is-success">{"This username is available"}</p>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Email"}</label>
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    class="input"
                                    type="email"
                                    placeholder="Email"
                                    value=&self.request.email
                                    oninput=oninput_email
                                    />
                                <span class="icon is-small is-left">
                                <i class="fas fa-envelope"></i>
                                </span>
                                <span class="icon is-small is-right">
                                <i class="fas fa-exclamation-triangle"></i>
                                </span>
                            </div>
                            // <p class="help is-danger">{"This email is invalid"}</p>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"Password"}</label>
                            <div class="control has-icons-left">
                                <input
                                    class="input"
                                    type="password"
                                    placeholder="Password"
                                    value=&self.request.password
                                    oninput=oninput_password
                                    />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-lock"></i>
                                </span>
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"You're supplier?"}</label>
                            <div class="control">
                                <label class="radio">
                                  <input type="radio" name="question"/>
                                  {1}
                                </label>
                                <label class="radio">
                                  <input type="radio" name="question"/>
                                  {0}
                                </label>
                            </div>
                        </fieldset>
                        <fieldset class="field">
                            <label class="label">{"What's CAD you use?"}</label>
                            <div class="control">
                                <div class="select">
                                  <select>
                                    <option>{1}</option>
                                    <option>{2}</option>
                                  </select>
                                </div>
                            </div>
                        </fieldset>
                        <div class="field">
                          <div class="control">
                            <label class="checkbox">
                              <input type="checkbox"/>
                              {" I agree to the "}<a href="#">{"terms and conditions"}</a>
                            </label>
                          </div>
                        </div>
                        <button
                            class="button"
                            type="submit"
                            disabled=false
                        >
                            { "Sign up" }
                        </button>
                    </fieldset>
                </form>
            </div>
        }
    }
}

// <div class="field">
//   <label class="label">Name</label>
//   <div class="control">
//     <input class="input" type="text" placeholder="Text input">
//   </div>
// </div>
//
// <div class="field">
//   <label class="label">Username</label>
//   <div class="control has-icons-left has-icons-right">
//     <input class="input is-success" type="text" placeholder="Text input" value="bulma">
//     <span class="icon is-small is-left">
//       <i class="fas fa-user"></i>
//     </span>
//     <span class="icon is-small is-right">
//       <i class="fas fa-check"></i>
//     </span>
//   </div>
//   <p class="help is-success">This username is available</p>
// </div>
//
// <div class="field">
//   <label class="label">Email</label>
//   <div class="control has-icons-left has-icons-right">
//     <input class="input is-danger" type="email" placeholder="Email input" value="hello@">
//     <span class="icon is-small is-left">
//       <i class="fas fa-envelope"></i>
//     </span>
//     <span class="icon is-small is-right">
//       <i class="fas fa-exclamation-triangle"></i>
//     </span>
//   </div>
//   <p class="help is-danger">This email is invalid</p>
// </div>
//
// <div class="field">
//   <label class="label">Subject</label>
//   <div class="control">
//     <div class="select">
//       <select>
//         <option>Select dropdown</option>
//         <option>With options</option>
//       </select>
//     </div>
//   </div>
// </div>
//
// <div class="field">
//   <label class="label">Message</label>
//   <div class="control">
//     <textarea class="textarea" placeholder="Textarea"></textarea>
//   </div>
// </div>
//
// <div class="field">
//   <div class="control">
//     <label class="checkbox">
//       <input type="checkbox">
//       I agree to the <a href="#">terms and conditions</a>
//     </label>
//   </div>
// </div>
//
// <div class="field">
//   <div class="control">
//     <label class="radio">
//       <input type="radio" name="question">
//       Yes
//     </label>
//     <label class="radio">
//       <input type="radio" name="question">
//       No
//     </label>
//   </div>
// </div>
//
// <div class="field is-grouped">
//   <div class="control">
//     <button class="button is-link">Submit</button>
//   </div>
//   <div class="control">
//     <button class="button is-link is-light">Cancel</button>
//   </div>
// </div>
