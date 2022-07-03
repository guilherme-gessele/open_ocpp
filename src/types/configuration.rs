use super::utils::{CiString500Type, CiString50Type};

/// Contains information about a specific configuration key.
/// It is returned in GetConfiguration.conf.
pub struct KeyValue {
    key: CiString50Type,
    /// False if the value can be set with the
    /// ChangeConfiguration message
    readonly: bool,
    /// If key is known but not set, this field may be absent.
    value: Option<CiString500Type>,
}
