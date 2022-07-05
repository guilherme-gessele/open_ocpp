use crate::types::enumerations::CancelReservationStatus;

/// CancelReservation.req PDU sent by the Central System to the
/// Charge Point.
pub struct CancelReservationReq {
    /// Id of the reservation to cancel.
    reservation_id: usize,
}

/// CancelReservation.conf PDU sent by the Charge Point to the
/// Central System in response to a CancelReservation.req PDU.
pub struct CancelReservationConf {
    /// This indicates the success or failure of the cancelling
    /// of a reservation by Central System.
    status: CancelReservationStatus,
}
