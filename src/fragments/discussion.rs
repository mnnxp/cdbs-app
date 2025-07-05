use std::collections::BTreeMap;

use chrono::NaiveDateTime;
use yew::{classes, html, Component, ComponentLink, FocusEvent, Html, InputData, Properties, ShouldRender};
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use log::debug;
use crate::error::Error;
use crate::fragments::list_errors::ListErrors;
use crate::fragments::user::GoToUser;
use crate::services::content_adapter::{DateDisplay, Markdownable};
use crate::services::{get_logged_user, get_value_field, resp_parsing};
use crate::types::{DiscussionCommentData, DiscussionInfo, ObjectType, ShowUserShort, SlimUser, ToObject, UUID};
use crate::get_gpl_to_object;
use crate::gqls::make_query;
use crate::gqls::discussion::{
    RegisterDiscussionComment, register_discussion_comment,
    GetDiscussions, get_discussions,
    GetDiscussionComments, get_discussion_comments,
    EditComment, edit_comment,
    DeleteComment, delete_comment,
};

use super::buttons::ft_delete_class_btn;

// Block for adding and displaying comments
pub struct DiscussionCommentsBlock {
    current_user: Option<SlimUser>,
    discussion: Option<DiscussionInfo>,
    replies: BTreeMap<UUID, Vec<DiscussionCommentData>>,
    new_comment: String,
    parent_comment_uuid: Option<UUID>,
    edit_comment: String,
    edit_comment_uuid: Option<UUID>,
    delete_comment_uuid: UUID,
    error: Option<Error>,
    props: Props,
    link: ComponentLink<Self>,
}

// Properties for DiscussionCommentsBlock
#[derive(Properties, Clone)]
pub struct Props {
    pub discussion_uuid: Option<UUID>,
    pub object_type: ObjectType,
}

// Msg for DiscussionCommentsBlock
#[derive(Clone)]
pub enum Msg {
    FetchDiscussions,
    FetchDiscussionsResult(String),
    FetchReplies(UUID),
    FetchRepliesResult(String),
    AddComment,
    AddCommentResult(String),
    ToReplyComment(UUID),
    UpdateNewComment(String),
    EditComment,
    EditCommentResult(String),
    ToEditComment(UUID, UUID, String),
    UpdateEditComment(String),
    DeleteComment(UUID, UUID),
    DeleteCommentResult(String),
    ClearReplies(UUID),
    ResetCommentFields,
    ResponseError(Error),
    ClearError,
}

// Impl for DiscussionCommentsBlock
impl Component for DiscussionCommentsBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            current_user: None,
            discussion: None,
            replies: BTreeMap::new(),
            new_comment: String::new(),
            parent_comment_uuid: None,
            edit_comment: String::new(),
            edit_comment_uuid: None,
            delete_comment_uuid: String::new(),
            error: None,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::FetchDiscussions);
            self.current_user = get_logged_user();
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        let link = self.link.clone();
        match msg {
            Msg::FetchDiscussions => {
                // Request for comments discussion
                let ipt_object_discussions_arg = get_discussions::IptObjectDiscussionsArg{
                    objectUuid: self.props.object_type.uuid.clone(),
                    toObject: get_gpl_to_object!(self.props.object_type.object_type, get_discussions::ToObject),
                    filterByUuids: None,
                };
                let ipt_sort = Some(get_discussions::IptSort {
                    byField: "createdAt".to_string(),
                    asDesc: true,
                });
                // let ipt_paginate = Some(get_discussions::IptPaginate {
                //     currentPage: self.page_set.current_page,
                //     perPage: self.page_set.per_page,
                // });
                spawn_local(async move {
                    let res = make_query(GetDiscussions::build_query(
                        get_discussions::Variables{
                            ipt_object_discussions_arg,
                            ipt_sort,
                            ipt_paginate: None
                        }
                    )).await.unwrap();
                    link.send_message(Msg::FetchDiscussionsResult(res));
                })
            },
            Msg::FetchDiscussionsResult(res) => {
                match resp_parsing::<Vec<DiscussionInfo>>(res, "discussions") {
                    Ok(result) => {
                        debug!("discussions: {:?}", result);
                        // Update list of comments
                        self.discussion = result.first().cloned();
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::FetchReplies(parent_uuid) => {
                // Request for answers in discussion comments
                let ipt_discussion_comments_arg = get_discussion_comments::IptDiscussionCommentsArg{
                    discussionUuid: self.discussion.as_ref().map(|di| di.uuid.clone()).unwrap_or("FFFFFFAAAIIIILLL".to_string()),
                    filterByParentUuid: Some(parent_uuid),
                    filterByUuids: None,
                };
                spawn_local(async move {
                    let res = make_query(GetDiscussionComments::build_query(
                        get_discussion_comments::Variables{
                            ipt_discussion_comments_arg,
                            ipt_sort: None,
                            ipt_paginate: None
                        }
                    )).await.unwrap();
                    link.send_message(Msg::FetchRepliesResult(res));
                })
            },
            Msg::FetchRepliesResult(res) => {
                match resp_parsing::<Vec<DiscussionCommentData>>(res, "discussionComments") {
                    Ok(result) => {
                        debug!("discussionComments: {:?}", result);
                        // Add replies to comments to the list of replies
                        if let Some(dcd) = result.first() {
                            self.replies.insert(dcd.parent_comment_uuid.clone(), result);
                        }
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::AddComment => {
                // Request to add new comments
                let ipt_discussion_comment_data = register_discussion_comment::IptDiscussionCommentData{
                    objectDiscussion: register_discussion_comment::IptDiscussionArg{
                        objectUuid: self.props.object_type.uuid.clone(),
                        toObject: get_gpl_to_object!(self.props.object_type.object_type, register_discussion_comment::ToObject),
                    },
                    discussionUuid: None, // self.discussion_uuid.clone()
                    parentCommentUuid: self.parent_comment_uuid.clone(),
                    messageContent: self.new_comment.clone()
                };
                spawn_local(async move {
                    let res = make_query(RegisterDiscussionComment::build_query(
                        register_discussion_comment::Variables{ipt_discussion_comment_data}
                    )).await.unwrap();
                    link.send_message(Msg::AddCommentResult(res));
                })
            },
            Msg::AddCommentResult(res) => {
                match resp_parsing::<UUID>(res, "registerDiscussionComment") {
                    Ok(result) => {
                        debug!("registerDiscussionComment: {:?}", result);
                        if let Some(pcu) = &self.parent_comment_uuid {
                            // Get a thread with replies to a comment
                            link.send_message(Msg::FetchReplies(pcu.clone()));
                        }
                        link.send_message(Msg::ResetCommentFields);
                        link.send_message(Msg::FetchDiscussions);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ToReplyComment(parent_uuid) => {
                // Get the thread with replies to the commented message
                link.send_message(Msg::FetchReplies(parent_uuid.clone()));
                // Set the UUID to reply to the comment
                self.parent_comment_uuid = Some(parent_uuid);
            },
            Msg::UpdateNewComment(new_comment) => {
                // Update the text of the new comment
                self.new_comment = new_comment;
            },
            Msg::EditComment => {
                if let Some(comment_uuid) = &self.edit_comment_uuid {
                    // Request for edit comment
                    let ipt_edit_comment_data = edit_comment::IptEditCommentData{
                        commentUuid: comment_uuid.clone(),
                        updatedMessage: self.edit_comment.clone(),
                    };
                    spawn_local(async move {
                        let res = make_query(EditComment::build_query(
                            edit_comment::Variables{ ipt_edit_comment_data }
                        )).await.unwrap();
                        link.send_message(Msg::EditCommentResult(res));
                    })
                }
            },
            Msg::EditCommentResult(res) => {
                match resp_parsing::<bool>(res, "editComment") {
                    Ok(result) => {
                        debug!("editComment: {:?}", result);
                        if let Some(edit_comment_uuid) = &self.edit_comment_uuid {
                            // if the comment is in the answer in a thread - update the thread
                            if let Some(parent_comment_uuid) = &self.parent_comment_uuid {
                                link.send_message(Msg::FetchReplies(parent_comment_uuid.clone()));
                            } else {
                                // Find a comment and update its message
                                match self.discussion.as_ref().map(|d| d.comments.iter().any(|c| &c.uuid == edit_comment_uuid)) {
                                    // if the comment is at the root - update the top part
                                    Some(true) => link.send_message(Msg::FetchDiscussions),
                                    _ => debug!("Problem with searching {} in comments and replies.", edit_comment_uuid),
                                }
                            }
                        }
                        // Clear UUID and fields
                        link.send_message(Msg::ResetCommentFields);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ToEditComment(comment_uuid, parent_comment_uuid, edit_comment) => {
                // Set the UUID to reply to the comment
                self.edit_comment_uuid = Some(comment_uuid.clone());
                // Set message content into an editable field
                self.edit_comment = edit_comment;
                match comment_uuid == parent_comment_uuid {
                    true => self.parent_comment_uuid = None,
                    // specify the parent of the edited comment to update the thread after the change
                    false => self.parent_comment_uuid = Some(parent_comment_uuid.clone()),
                }
            },
            Msg::UpdateEditComment(edit_comment) => {
                // Change the text of the old comment
                self.edit_comment = edit_comment;
            },
            Msg::DeleteComment(comment_uuid, parent_comment_uuid) => {
                if self.delete_comment_uuid != comment_uuid {
                    self.delete_comment_uuid = comment_uuid.clone();
                    return true;
                }
                match comment_uuid == parent_comment_uuid {
                    true => self.parent_comment_uuid = None,
                    false => {
                        // specify the parent of the deteled comment to update the thread after the change
                        self.parent_comment_uuid = Some(parent_comment_uuid.clone());
                        // clear old replies to this parent comment
                        self.replies.remove(&parent_comment_uuid);
                    },
                }
                // Request for delete comment
                spawn_local(async move {
                    let res = make_query(DeleteComment::build_query(
                        delete_comment::Variables{ comment_uuid }
                    )).await.unwrap();
                    link.send_message(Msg::DeleteCommentResult(res));
                })
            },
            Msg::DeleteCommentResult(res) => {
                match resp_parsing::<bool>(res, "deleteComment") {
                    Ok(result) => {
                        debug!("deleteComment: {:?}", result);
                        // if the comment is in the answer in a thread - update the thread
                        if let Some(parent_comment_uuid) = &self.parent_comment_uuid {
                            link.send_message(Msg::FetchReplies(parent_comment_uuid.clone()));
                        } else {
                            link.send_message(Msg::FetchDiscussions)
                        }
                        // Clear UUID and fields
                        link.send_message(Msg::ResetCommentFields);
                    },
                    Err(err) => link.send_message(Msg::ResponseError(err)),
                }
            },
            Msg::ClearReplies(parent_uuid) => {
                // Delete comment replies thread (clear)
                self.replies.remove(&parent_uuid);
            },
            Msg::ResetCommentFields => {
                // Clear UUID for reply to comment
                self.parent_comment_uuid = None;
                // Clear field for new comment
                self.new_comment.clear();
                // Clear UUID for edit comment
                self.edit_comment_uuid = None;
                // Clear field for edit comment
                self.edit_comment.clear();
                // Clear UUID for flag confirm delete comment
                self.delete_comment_uuid.clear();
            },
            Msg::ResponseError(err) => self.error = Some(err),
            Msg::ClearError => self.error = None,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Update discussion_uuid if changed
        if self.props.object_type.uuid == props.object_type.uuid &&
            self.props.discussion_uuid == props.discussion_uuid {
            false
        } else {
            self.props = props;
            // Update comments for new discussion
            self.link.send_message(Msg::FetchDiscussions);
            true
        }
    }

    fn view(&self) -> Html {
        let onclick_clear_error = self.link.callback(|_| Msg::ClearError);
        html! {
            <div class="container">
                <ListErrors error={self.error.clone()} clear_error={onclick_clear_error.clone()}/>
                {match &self.parent_comment_uuid {
                    Some(_) => html!{},
                    None => self.view_form_add(),
                }}
                <ul class="menu-list">
                    {match &self.discussion {
                        Some(discuss) => html!{ for discuss.comments.iter().map(|comment|
                            self.view_comment(
                                &comment.uuid,
                                &comment.parent_comment_uuid,
                                &comment.message_content,
                                &comment.author,
                                &comment.replies_count,
                                &comment.created_at,
                            ))
                        },
                        None => html!{},
                    }}
                </ul>
            </div>
        }
    }
}

// View for one comment
impl DiscussionCommentsBlock {
    fn view_comment(
        &self,
        comment_uuid: &UUID,
        parent_comment_uuid: &UUID,
        message_content: &str,
        author: &ShowUserShort,
        replies_count: &usize,
        created_at: &NaiveDateTime,
    ) -> Html {
        html! {
            <li class="">
                <article id={format!("comment-{}", comment_uuid)} class="comment mt-5 box">
                    <div class="comment-info">
                        <div class="comment-avatar">
                            <img src={author.image_file.download_url.clone()} alt="Author"/>
                        </div>
                        <div class="comment-username mr-1">
                            <GoToUser data = {author.clone()} />
                            <span>{format!("{} {} ", &author.firstname, &author.lastname)}</span>
                        </div>
                        <div class="columns is-mobile right-side">
                            <div class="comment-time has-text-grey-light">{created_at.date_to_display()}</div>
                            {match &self.current_user {
                                Some(slim_user) if slim_user.uuid == author.uuid => html!{
                                    <div class="buttons">
                                        {self.to_edit_btn(comment_uuid.clone(), parent_comment_uuid.clone(), &message_content)}
                                        {self.to_delete_btn(comment_uuid.clone(), parent_comment_uuid.clone())}
                                    </div>
                                },
                                _ => html!{},
                            }}
                        </div>
                    </div>
                    <div class="comment-body">
                        <div class="comment-spoiler">
                            <div class="comment-text">
                                <p>{ self.message_block(comment_uuid, message_content) }</p>
                            </div>
                        </div>
                        <div class="comment-actions buttons right-side">
                            {self.view_comment_actions(&comment_uuid, replies_count)}
                            {self.to_reply_btn(comment_uuid.clone())}
                        </div>
                    </div>
                </article>
                {self.to_reply_form(comment_uuid.clone())}
                {self.view_replies(&comment_uuid)}
            </li>
        }
    }

    fn view_comment_actions(&self, comment_uuid: &UUID, replies_count: &usize) -> Html {
        let replies_comment_uuid = comment_uuid.to_string();
        let clear_replies_for_uuid = comment_uuid.to_string();
        let onclick_fetch_replies = self.link.callback(move |_| Msg::FetchReplies(replies_comment_uuid.clone()));
        let onclick_clear_replies = self.link.callback(move |_| Msg::ClearReplies(clear_replies_for_uuid.clone()));
        match self.replies.get(comment_uuid) {
            Some(_) => html! {<>
                <button class="button is-small" onclick={onclick_fetch_replies}>
                    <span class={"icon"} style={"color: #1872f0;"}>
                        <i class="fas fa-sync" aria-hidden={"true"}></i>
                    </span>
                </button>
                <button class="button is-small" onclick={onclick_clear_replies}>
                    { get_value_field(&301) }
                </button>
            </>},
            None if replies_count > &0 => html!{
                <button class="button is-small is-info" onclick={onclick_fetch_replies}>
                    { format!{"{} ({})", get_value_field(&383), replies_count} }
                </button>
            },
            None => html!{},
        }
    }

    // View for replies to comments
    fn view_replies(&self, comment_uuid: &UUID) -> Html {
        match self.replies.get(comment_uuid) {
            Some(replies) => html!{
                <ul>
                    {for replies.iter().map(|reply|
                        self.view_comment(
                            &reply.uuid,
                            &reply.parent_comment_uuid,
                            &reply.message_content,
                            &reply.author,
                            &reply.replies_count,
                            &reply.created_at
                        )
                    )}
                </ul>
            },
            None => html!{},
        }
    }

    // View for entering a comment or replying to a comment
    fn view_form_add(&self) -> Html {
        let onsubmit_add_comment = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::AddComment
        });
        let oninput_update_new_comment = self.link.callback(|ev: InputData| Msg::UpdateNewComment(ev.value));
        let onclick_clear_reply = self.link.callback(|_| Msg::ResetCommentFields);
        let (class_btns, class_add_btn) = match &self.parent_comment_uuid {
            Some(_) => ("buttons is-right pt-1", "button is-small is-primary"),
            None => ("buttons pt-1", "button is-small is-primary is-fullwidth"),
        };
        html! {
            <div class="box">
                <form onsubmit={onsubmit_add_comment}>
                    <textarea
                        class="textarea"
                        placeholder={get_value_field(&384)}
                        value={self.new_comment.clone()}
                        oninput={oninput_update_new_comment}
                    />
                    <div class={class_btns}>
                        {match &self.parent_comment_uuid {
                            Some(_) => html!{<>
                                <button class={class_add_btn} type="submit" style="min-width: 25%;">{ get_value_field(&382) }</button>
                                <button class="button is-small is-warning" onclick={onclick_clear_reply}>{ get_value_field(&221) }</button>
                            </>},
                            None => html!{<button class={class_add_btn} type="submit">{ get_value_field(&381) }</button>},
                        }}
                    </div>
                </form>
            </div>
        }
    }

    // View to display the message entry form for a reply to reply to a comment
    fn to_reply_form(&self, comment_uuid: UUID) -> Html {
        match (&self.parent_comment_uuid, self.edit_comment_uuid.is_none()) {
            (Some(pcu), true) if pcu == &comment_uuid => self.view_form_add(),
            _ => html! {},
        }
    }

    // View to display a button for opening a message entry form for a reply to reply to a comment
    fn to_reply_btn(&self, comment_uuid: UUID) -> Html {
        match &self.parent_comment_uuid {
            Some(pcu) if pcu == &comment_uuid => html!{},
            _ => html! {
                <button
                    class="button is-small is-info"
                    onclick={self.link.callback(move |_| Msg::ToReplyComment(comment_uuid.clone()))}>
                    { get_value_field(&382) }
                </button>
            },
        }
    }

    // View to display the message edit form for a reply to reply to a comment
    fn message_block(&self, comment_uuid: &UUID, message_content: &str) -> Html {
        match &self.edit_comment_uuid.as_ref().map(|ec_uuid| ec_uuid == comment_uuid) {
            Some(true) => self.view_form_edit(),
            _ => html!{message_content.to_markdown()},
        }
    }

    // View to display a button for opening a message edit form for a reply to reply to a comment
    fn to_edit_btn(&self, comment_uuid: UUID, parent_comment_uuid: UUID, message_content: &str) -> Html {
        let e_message_content = message_content.to_string();
        let onclick_edit_btn = self.link.callback(move |_| {
            Msg::ToEditComment(comment_uuid.clone(), parent_comment_uuid.clone(), e_message_content.clone())
        });
        let disabled_btn = self.edit_comment_uuid.is_some();
        html!{
            <button class="button is-small" onclick={onclick_edit_btn} disabled={disabled_btn} >
                <span class="icon">
                <i aria-hidden="true" class="fas fa-edit"></i>
                </span>
                <span>{get_value_field(&334)}</span>
            </button>
        }
    }

    // View to edit a comment or reply to a comment
    fn view_form_edit(&self) -> Html {
        let onsubmit_edit_comment = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::EditComment
        });
        let oninput_update_edit_comment = self.link.callback(|ev: InputData| Msg::UpdateEditComment(ev.value));
        let onclick_clear_reply = self.link.callback(|_| Msg::ResetCommentFields);
        html! {
            <div class="box">
                <form onsubmit={onsubmit_edit_comment}>
                    <textarea
                        class="textarea"
                        placeholder={self.edit_comment.clone()}
                        value={self.edit_comment.clone()}
                        oninput={oninput_update_edit_comment}
                    />
                    <div class={"buttons is-right pt-1"}>
                        <button class={"button is-small is-primary"} type="submit" style="min-width: 25%;">{ get_value_field(&46) }</button>
                        <button class="button is-small is-warning" onclick={onclick_clear_reply}>{ get_value_field(&221) }</button>
                    </div>
                </form>
            </div>
        }
    }

    // View for displaying the delete comment button
    fn to_delete_btn(&self, comment_uuid: UUID, parent_comment_uuid: UUID) -> Html {
        let confirm = self.delete_comment_uuid == comment_uuid;
        let onclick_delete_btn = self.link.callback(move |_| {
            Msg::DeleteComment(comment_uuid.clone(), parent_comment_uuid.clone())
        });
        ft_delete_class_btn("delete-comment", onclick_delete_btn, confirm, false, classes!("is-small"))
    }
}