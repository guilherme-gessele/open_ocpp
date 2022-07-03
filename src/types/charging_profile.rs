use chrono::{DateTime, Utc};

use super::charging_schedule::ChargingSchedule;

use super::enumerations::{
    ChargingProfileKindType, ChargingProfilePurposeType, RecurrencyKindType,
};

/// A ChargingProfile consists of a ChargingSchedule, describing
/// the amount of power or current that can be delivered
/// per time interval.
pub struct ChargingProfile {
    /// Unique identifier for this profile.
    charging_profile_id: usize,
    /// Only valid if ChargingProfilePurpose is set
    /// to TxProfile, the transactionId MAY be used to
    /// match the profile to a specific transaction.
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
