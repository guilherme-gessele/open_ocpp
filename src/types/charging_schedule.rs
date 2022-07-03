use chrono::{DateTime, Utc};

use super::charging_schedule_period::ChargingSchedulePeriod;
use super::enumerations::ChargingRateUnitType;

pub struct ChargingSchedule {
    /// Duration of the charging schedule in seconds.
    /// If the duration is left empty, the last period
    /// will continue indefinitely or unitl end of the
    /// transaciton in case startSchedule is absent.
    duration: Option<usize>,
    /// Starting point of an absolude scheduel.
    /// If absent the schedule will be relative to
    /// start of charging.
    start_schedule: Option<DateTime<Utc>>,
    /// The unit of measure Limit is expressed in.
    charging_rate_unit: ChargingRateUnitType,
    /// List of ChargingSchedulePeriod elements defining
    /// maximum power of current usage over time.
    charging_schedule_period: Vec<ChargingSchedulePeriod>,
    /// Minimum charging rate supported bu the electric vehicle.
    /// The unit of measure is defined by the chargingRateUnit.
    /// This parameter is intended to be used by a local smart charging
    /// algorithm to optimize the power allocation for in the case
    /// a charging process is inefficient at lower charging rates.
    /// Accepts at most one digit fraction.
    min_charging_rate: Option<f64>,
}
