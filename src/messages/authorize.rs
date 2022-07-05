use crate::types::authorization::{IdTagInfo, IdToken};

/// Authorize.req PDU sent by the Charge Point to the Central System.
pub struct AuthorizeReq {
    /// This contains the identifier
    /// that needs to be authorized.
    id_tag: IdToken,
}

/// Authorize.conf PDU sent by the Central System
/// to the Charge Point in response to a Authorize.req PDU.
pub struct AuthorizeConf {
    /// This contains information about authorization status,
    /// expiry and parent id.
    id_tag_info: IdTagInfo,
}
