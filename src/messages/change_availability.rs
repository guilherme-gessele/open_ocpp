use crate::types::enumerations::{AvailabilityStatus, AvailabilityType};

/// ChangeAvailability.req PDU sent by the Central System to the
/// Charge Point.
pub struct ChangeAvailabilityReq {
    /// The id of the connector for which availability
    /// needs to change. Id '0' (zero) is used if the
    /// availability of the Charge Point and all its
    /// connectors need to change.
    connector_id: usize,
    /// This contains the type of availability change
    /// that the Charge Point should perform.
    availability_type: AvailabilityType,
}

/// ChangeAvailability.conf PDU return by Charge Point to
/// Central System.
pub struct ChangeAvailabilityConf {
    /// This indicates whether the Charge Point is able
    /// to perform the availability change.
    status: AvailabilityStatus,
}
