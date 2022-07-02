use chrono::{DateTime, Utc};

use super::enumerations::AuthorizationStatus;
use super::id_token::IdToken;

pub struct IdTagInfo {
    expiry_date: Option<DateTime<Utc>>,
    parent_id_tag: Option<IdToken>,
    status: AuthorizationStatus,
}

impl IdTagInfo {}
