use crate::types::enumerations::FirmwareStatus;

/// FirmwareStatusNotification.req PDU sent by
/// the Charge Point to the Central System.
pub struct FirmwareStatusNotificationReq {
    /// This contains the progress status of the
    /// firmware installation.
    status: FirmwareStatus,
}
