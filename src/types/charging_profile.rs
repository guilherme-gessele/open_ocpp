use chrono::{DateTime, Utc};

use super::enumerations::{
    ChargingProfileKindType, ChargingProfilePurposeType, ChargingRateUnitType, RecurrencyKindType,
};

/// A ChargingProfile consists of a ChargingSchedule, describing
/// the amount of power or current that can be delivered
/// per time interval.
pub struct ChargingProfile {
    /// Unique identifier for this profile.
    charging_profile_id: usize,
    /// Only valid if ChargingProfilePurpose is set
    /// to TxProfile, the transactionId MAY be used to
    /// match the profile to a specific transaction_id.
    transaction_id: Option<usize>,
    /// Value determinig level in hierarchy stack of profiles.
    /// Higher values have precedence over lower values.
    /// Lowest level is 0.
    stack_level: usize,
    charging_profile_purpose: ChargingProfilePurposeType,
    /// Indicates the kind of schedule.
    charging_profile_king: ChargingProfileKindType,
    /// Indicates the start point of a recurrence.
    recurrency_kind: Option<RecurrencyKindType>,
    /// Point in time at which the profile stops to be valid.
    /// If absent, the profile is valid until it is replaced
    /// by another profile. Not to be used when ChargingProfilePurpose
    /// is TxProfile.
    valid_from: Option<DateTime<Utc>>,
    /// Contains limits for the available power or current over time.
    charging_schedule: ChargingSchedule,
}

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

pub struct ChargingSchedulePeriod {
    /// Start of the period, in seconds from the
    /// start of schedule. The value of StartPeriod
    /// also defines the stop time of the previous
    /// period.
    start_period: usize,
    /// Power limit during the schedule period, expressed in
    /// Amperes. Accepts at most one digit fraction.
    limit: f64,
    /// The number of phases that can be used for charging.
    /// If a number of phases is needed, numberPhases = 3
    /// will be assumed unless another number is given.
    number_phases: Option<usize>,
}
