use crate::types::utils::CiString50Type;

use crate::types::configuration::KeyValue;

/// GetConfiguration.req PDU  sent by the Central System
/// to the Charge Point.
pub struct GetConfigurationReq {
    /// List of keys for which the configuration
    /// value is required.
    key: Option<Vec<CiString50Type>>,
}

/// GetConfiguration.conf PDU sent by Charge Point to the
/// Central System in response to a GetConfiguration.req.
pub struct GetConfigurationConf {
    /// List of requested or known keys.
    configuration_key: Option<Vec<KeyValue>>,
    /// Requested keys that are unknown.
    unknown_key: Option<Vec<CiString50Type>>,
}
