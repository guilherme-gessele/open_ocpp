use chrono::{DateTime, Utc};

use crate::types::enumerations::{ChargingRateUnitType, GetCompositeScheduleStatus};

use crate::types::charging_profile::ChargingSchedule;

/// GetCompositeSchedule.req PDU sent by the Central System
/// to the Charge Point.
pub struct GetCompositeScheduleReq {
    /// The ID of the Connector for which the schedule is requested.
    /// When ConnectorId = 0, the Charge Point will calculate the
    /// expected consumption for the grid connection.
    connector_id: usize,
    /// Time in seconds. Length of requested schedule.
    duration: usize,
    /// Can be used to force a power or current profile.
    charging_rate_unit: Option<ChargingRateUnitType>,
}

/// GetCompositeSchedule.conf PDU sent by the Charge Point
/// to the Central System in response to a GetCompositeSchedule.req PDU.
pub struct GetCompositeScheduleConf {
    /// Status of the request.
    /// The Charge Point will indicate if it was able
    /// to process the request.
    status: GetCompositeScheduleStatus,
    /// The charging schedule contained in this notification
    /// applies to a Connector.
    connector_id: Option<usize>,
    /// Time. Periods contained in the charging profile are
    /// relative to this point in time.
    schedule_start: Option<DateTime<Utc>>,
    /// Planned Composite Charging Schedule, the energy
    /// consumption over time.
    /// Always relative to ScheduleStart.
    charging_schedule: Option<ChargingSchedule>,
}
