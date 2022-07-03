/// Status in a response to an Authorize.req
pub enum AuthorizationStatus {
    /// Identifier is allowed for charging
    Accepted,
    /// Identifier has been blocked. Not allowed for charging.
    Blocked,
    /// Identifier has expired. Not allowed for charging.
    Expired,
    /// Identifier is unknown. Not allowed for charging
    Invalid,
    /// Identifier is already involved in another transaction and multiple
    /// transactions are not allowed.
    ConcurrentTx,
}

/// Status returned in response to ChangeAvailability.req
pub enum AvailabilityStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Request has been accepted and will be executed when transactions(s)
    /// in progress have finished.
    Scheduled,
}

/// Request availability change in ChangeAvailability.req
pub enum AvailabilityType {
    /// Charge point is not available for charging.
    Inoperative,
    /// Charge point is available for charging.
    Operative,
}

/// Status in CancelReservation.conf
pub enum CancelReservationStatus {
    /// Reservation for the identifier has been cancelled.
    Accepted,
    /// Reservation could not be cancelled, because there is no reservation
    /// active for the identifier.
    Rejected,
}

/// Charge Point status reported in StatusNotification.req
pub enum ChargePointErrorCode {
    /// Failure to lock or unlock connector.
    ConnectorLockFailure,
    /// Communication failure with the vehicle, migh be Mode 3 or other
    /// communication protocol problem. This is not a real error in the
    /// sense that the Charge Point doesn't need to go to the faulted
    /// state. Instead, it should go to the SuspendedEVSE state.
    EVCommunicationError,
    /// Ground fault circuit interrupter has been activated.
    GroundFailure,
    /// Temperature inside Charge Point is too high.
    HighTemperature,
    /// Error in internal hard- or software component.
    InternalError,
    /// The authorization information received from the Central System
    /// is in conflict with the LocalAuthorizationList.
    LocalListConflict,
    /// No error to report.
    NoError,
    /// Other type of error. More information in vendorErrorCode.
    OtherError,
    /// Over current protection device has tripped.
    OverCurrentFailure,
    /// Voltage has risen above an acceptable level.
    OverVoltage,
    /// Failure to read power meter.
    PowerMeterFailure,
    /// Failure to control power switch.
    PowerSwitchFailure,
    /// Failure with idTag reader.
    ReaderFailure,
    /// Unable to perform a reset.
    ResetFailure,
    /// Voltage has dropped below an acceptable level.
    UnderVoltage,
    /// Wireless communication device reports a weak signal.
    WeakSignal,
}

/// Status reported in StatusNotification.req. A status can be reported for the
/// Charge Point main controller (connectorId = 0) or for a specific connector.
/// Status for the Charge Point main controller is a subset of the enumeration:
/// Available, Unavailable or Faulted.
///
/// Stated considered Operative are: Available, Preparing, Charging, SuspendedEVSE,
/// SuspendedEV, Finishing, Reserved. States considered Inoperative are:
/// Unavailable, Faulted.
pub enum ChargePointStatus {
    /// When a Connector becomes available for a new user. (Operative)
    Available,
    /// When a Connector becomes no longer available for a new user
    /// but no charging session is active. Typically a Connector is
    /// occupied when a user presents a tag, inserts a cable or a
    /// vehicle occupies the parking bay. (Operative)
    Preparing,
    /// When the contactor of a Connector closes, allowing the
    /// vehicle to charge. (Operative)
    Charging,
    /// When the contactor of a Connector opens upon request of the
    /// EVSE, e.g. due to a smart charging restriction or as the result
    /// of StartTransaction.conf indicating that charging is not allowed.
    /// (Operative)
    SuspendedEVSE,
    /// When the EVSE is ready to deliver energy but contactor is open,
    /// e.g. the EV is not ready. (Operative)
    SuspendedEV,
    /// When a charging session has stopped at a Connector, but the Connector
    /// is not yet available for a new user, e.g. the cable has not been removed
    /// or the vehicle has not left the parking bay. (Operative)
    Finishing,
    /// When a Connector becomes reserved as a result of a Reserve Now command.
    /// (Operative)
    Reserved,
    /// When a Conenctor becomes unavailable as the result of a Change Availability
    /// command or an event upon which the Charge Point transaitions to unavailable
    /// at its discretion. Upon receipt of a Chage Availability command, the status
    /// MAY be scheduled. When scheduled, the Status Nofitication shall be send when
    /// the availability change becomes effective. (Inoperative)
    Unavailable,
    /// When a Charge Point or connector has reported an error and is not available
    /// for energy delivery. (Inoperative)
    Faulted,
}

pub enum ChargingProfileKindType {
    /// Schedule periods are relative to a fixed point in time defined in the schedule.
    Absolute,
    /// The schedule restarts periodically at the first schedule period.
    Recurring,
    /// Schedule periods are relative to a situation-specific start point (such as the
    /// start of a session) that is determined by the charge point.
    Relative,
}

pub enum ChargingProfilePurposeType {
    /// Configuration for the maximum power or current available for an entire Charge Point.
    /// SetChargingProfile.req message.
    ChargePointMaxProfile,
    /// Default profile to be used for new transactions.
    TxDefaultProfile,
    /// Profile with constraints to be imposed by the Charge Point on the current transaction.
    /// A profile with this purpose SHALL cease to be valid when the transaction terminates.
    TxProfile,
}

/// Status returned in response to SetChargingProfile.req.
pub enum ChargingProfileStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Charge Point indicates that the request is not supported.
    NotSupported,
}

/// Unit in which a charging schedule is defined, as used in: GetCompositeSchedule.req and
/// ChargingSchedule.
pub enum ChargingRateUnitType {
    /// Watts (power).
    W,
    /// Amperes (current).
    A,
}

pub enum RecurrencyKindType {
    /// The schedule restarts at the beginning of the next day.
    Daily,
    /// The schedule restarts at the beginning of the next week (defined as
    /// Monday morning).
    Weekly,
}
