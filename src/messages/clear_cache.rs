use crate::types::enumerations::ClearCacheStatus;

/// ClearCache.conf PDU sent by the Charge Point to the
/// Central System.
pub struct ClearCacheConf {
    /// Accepted if the Charge Point has executed
    /// the request, otherwise rejected.
    status: ClearCacheStatus,
}
