use crate::types::enumerations::{ChargingProfilePurposeType, ClearChargingProfileStatus};

/// ClearChargingProfile.req PDU sent by the Central System
/// to the Charge Point.
///
/// The Central System can use this message to clear (remove)
/// either a specific charging profile (denoted by id) or a
/// selection of charging profiles that match with the values
/// of the optional connector_id, stack_level, and
/// charging_profile_purpose fields.
pub struct ClearChargingProfileReq {
    /// The ID of the charging profile to clear.
    id: Option<usize>,
    /// Specifies the ID of the connector for which to
    /// clear charging profiles. A connectorId of zero (0)
    /// specifies the charging profile for the overall
    /// Charge Point.
    ///
    /// Absence of this parameter means the clearing applies
    /// to all charging profiles that match the other criteria
    /// in the request.
    connector_id: Option<usize>,
    /// Specifies to purpose of the charging profiles that
    /// will be cleared, if they meet the other criteria
    /// in the request.
    charging_profile_purpose: Option<ChargingProfilePurposeType>,
    /// Specifies the stackLevel for which charging profiles
    /// will be cleared, if they meet the other criteria in
    /// the request.
    stack_level: Option<usize>,
}

/// ClearChargingProfile.conf PDU sent by the Charge Point to the
/// Central System in response to a ClearChargingProfile.req PDU.
pub struct ClearChargingProfileConf {
    /// Indicates if the Charge Point was able to execute the request.
    status: ClearChargingProfileStatus,
}
