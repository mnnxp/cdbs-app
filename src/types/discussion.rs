use super::profiles::ShowUserShort;
use super::UUID;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct ObjectType {
    pub uuid: UUID,
    pub object_type: ToObject,
}

impl ObjectType {
    pub fn new(uuid: UUID, object_type: ToObject) -> Self {
        ObjectType { uuid, object_type }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum ToObject {
    COMPANY,
    COMPONENT,
    SERVICE,
}

/// Macro for converting `ToObject` to a specific `ToObject` type for GraphQL requests.
///
/// This macro takes two arguments:
/// - `$value`: an expression of type `ToObject` to be converted.
/// - `$target_type`: the type to which the value should be converted.
///
/// The macro generates code to convert the value to the specified type using a match on the `ToObject` variants.
///
/// # Example usage
///
/// ```rust
/// let to_object = get_gql_to_object!(ToObject::COMPANY, crate::gqls::discussion::get_discussions::ToObject);
/// ```
#[macro_export]
macro_rules! get_gql_to_object {
    ($value:expr, $target_type:ty) => {
        match $value {
            ToObject::COMPANY => <$target_type>::COMPANY,
            ToObject::COMPONENT => <$target_type>::COMPONENT,
            ToObject::SERVICE => <$target_type>::SERVICE,
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscussionInfo {
    pub uuid: UUID,
    pub title: String,
    pub is_pinned: bool,
    pub last_activity_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub replies_count: usize,
    pub comments: Vec<DiscussionCommentData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscussionCommentData {
    pub uuid: UUID,
    pub discussion_uuid: UUID,
    pub parent_comment_uuid: UUID,
    pub author: ShowUserShort,
    pub message_content: String,
    pub is_edited: bool,
    pub is_hidden: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub replies_count: usize,
    // pub replies: Vec<DiscussionCommentData>,
}
