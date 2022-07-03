use super::id_tag_info::IdTagInfo;
use super::id_token::IdToken;

pub struct AuthorizationData {
    /// The identifier to with this authorization applies
    id_tag: IdToken,
    /// This contains information about authorization status,
    /// expiry and parent id. For a differential update the
    /// following applies: If this element is present, then
    /// this entry SHALL be added or updated in the Local
    /// Authorization List. If this element is absent, than
    /// the Local Authorization List SHALL be deleted.
    id_tag_info: Option<IdTagInfo>,
}
