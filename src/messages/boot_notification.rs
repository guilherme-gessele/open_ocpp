use chrono::{DateTime, Utc};

use crate::types::enumerations::RegistrationStatus;

use crate::types::utils::{CiString20Type, CiString25Type, CiString50Type};

/// BootNotification.req PDU sent by the Charge Point to the
/// Central System.
pub struct BootNotificationReq {
    /// This contains a value that identifies the
    /// serial number of the Charge Box inside
    /// the Charge Point.
    /// Deprecated, will be removed in future version.
    charge_box_serial_number: Option<CiString25Type>,
    /// This contains a value that identifies the model of the Charge Point.
    charge_point_model: CiString20Type,
    /// This contains a value that identifies the
    /// serial number of the Charge Point.
    charge_point_serial_number: Option<CiString25Type>,
    /// This contains a value that identifies
    /// the vendor of the ChargePoint.
    charge_point_vendor: CiString20Type,
    /// This contains the firmware version of the Charge Point.
    firmware_version: Option<CiString50Type>,
    /// This contains the ICCID of the modem's SIM card.
    iccid: Option<CiString20Type>,
    /// This contains the IMSI of the modem's SIM card.
    imsi: Option<CiString20Type>,
    /// This contains the serial number of the main power meter
    /// of the Charge Point.
    meter_serial_number: Option<CiString25Type>,
    /// This contains the type of the main power meter
    /// of the Charge Point.
    meter_type: Option<CiString25Type>,
}

/// BootNotification.conf PDU sent by the Central System
/// to the Charge Point in response to a BootNotification.req PDU.
pub struct BootNotificationConf {
    /// This contains the Central System's current time.
    current_time: DateTime<Utc>,
    interval: usize,
    status: RegistrationStatus,
}
