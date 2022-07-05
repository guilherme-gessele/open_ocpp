use crate::types::enumerations::DiagnosticsStatus;

/// DiagnosticsStatusNotification.req PDU sent by the Charge Point
/// to the Central System.
pub struct DiagnosticsStatusNotificationReq {
    /// This contains the status of the diagnostics upload.
    status: DiagnosticsStatus,
}
