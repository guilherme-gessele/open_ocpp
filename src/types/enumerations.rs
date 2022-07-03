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

/// Status returned in response to ClearCache.req.
pub enum ClearCacheStatus {
    /// Command has been executed.
    Accepted,
    /// Command has not been executed.
    Rejected,
}

/// Status returned in response to ClearChargingProfile.req.
pub enum ClearChargingProfileStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// No CHarging Profile(s) were found matching the request.
    Unknown,
}

/// Status in ChangeConfiguration.conf.
pub enum ConfigurationStatus {
    /// Configuration key supported and setting has been changed.
    Accepted,
    /// Configuration key supported, but setting could not be changed.
    Rejected,
    /// Configuration key supported and setting has been changed,
    /// bug change will be available after reboot (Charge Point
    /// will not reboot itself).
    RebootRequired,
    /// Configuration key is not supported.
    NotSupported,
}

/// Status in DataTransfer.conf.
pub enum DataTransferStatus {
    /// Message has been accepted and the contained request is accepted.
    Accepted,
    /// Message has been accepted but the contained request is rejected.
    Rejected,
    /// Message could not be interpreted due to unknown messageId string.
    UnknownMessageId,
    /// Message could not be interpreted due to unknown vendorId string.
    UnknownVendorId,
}

/// Status in DiagnosticsStatusNotification.req.
pub enum DiagnosticsStatus {
    /// Charge Point is not performing diagnostics related tasks.
    /// Status Idle SHALL only be used as in a DiagnosticsStatusNotification.req
    /// that was triggered by a TriggerMessage.req.
    Idle,
    /// Diagnostics information has been uploaded.
    Uploaded,
    /// Uploading of diagnostics failed.
    UploadFailed,
    /// File is being uploaded.
    Uploading,
}

/// Status of firmware download as reported in FirmwareStatusNotification.req.
pub enum FirmwareStatus {
    /// New firmware has been downloaded by Charge Point.
    Downloaded,
    /// Charge point failed to download firmware.
    DownloadFailed,
    /// Firmware is being downloaded.
    Downloading,
    /// Charge Point is not performing fimrware update related tasks.
    /// Status Idle SHALL only be used as in a FirmwareStatusNotification.req
    /// that was triggered by a TriggerMessage.req.
    Idle,
    /// Installation of new firmware has failed.
    InstallationFailed,
    /// Firmware is being installed.
    Installing,
    /// New firmware has successfully been installed in charge point.
    Installed,
}

/// Status returned in response to GetCompositeSchedule.req.
pub enum GetCompositeScheduleStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
}

/// Allowable values of the optional "location" field of a value element in SampledValue.
pub enum Location {
    /// Measurement inside body of Charge Point (e.g Temperature).
    Body,
    /// Measurement taken from cable between EV and Charge Point.
    Cable,
    /// Measurement taken by EV.
    EV,
    /// Measurement at network ("grid") inlet connection.
    Inlet,
    /// Measurement at a Connector. Default value.
    Outlet,
}

/// Allowable values of the optional "measurand" field of a Value element,
/// as used in MeterValues.req and StopTransaction.req messages.
/// Default value of "measurand" is always "Energy.Active.Import.Register".
pub enum Measurand {
    /// Instantaneous current flow from EV.
    CurrentExport,
    /// Instantaneous current flow to EV.
    CurrentImport,
    /// Maximum current offered to EV.
    CurrentOffered,
    /// Energy exported by EV (Wh or kWh).
    EnergyActiveExportRegister,
    /// Enegy imported by EV (Wh or kWh).
    EnergyActiveImportRegister,
    /// Reactive energy exported by EV (varh or kvarh).
    EnergyReactiveExportRegister,
    /// Reactive energy imported by EV (varh or kvarh).
    EnergyReactiveImportRegister,
    /// Energy exported by EV (Wh or kWh).
    EnergyActiveExportInterval,
    /// Energy imported by EV (Wh or kWh).
    EnergyActiveImportInterval,
    /// Reactive energy exported by EV (varh or kvarh).
    EnergyReactiveExportInterval,
    /// Reactive energy imported by EV (varh or kvarh).
    EnergyReactiveImportInterval,
    /// Instantaneous reading of powerline frequency.
    Frequency,
    /// Instantaneous active power exported by EV (W or kW).
    PowerActiveExport,
    /// Instantaneous active power imported by EV (W or kW).
    PowerActiveImport,
    /// Instantaneous power factor of total energy flow.
    PowerFactor,
    /// Maximum power offered to EV.
    PowerOffered,
    /// Instantaneous reactive power exported by EV (var or kvar).
    PowerReactiveExport,
    /// Instantaneous reactive power imported by EV (var or kvar).
    PowerReactiveImport,
    /// Fan speed in RPM.
    RPM,
    /// State of charge of charging vehicle in percentage.
    SoC,
    /// Temperature reading inside Charge Point.
    Temperature,
    /// Instantaneous AC RMS supply voltage.
    Voltage,
}

/// Type of request to be triggered in a TriggerMessage.req.
pub enum MessageTrigger {
    /// To trigger a BootNotification request.
    BootNotification,
    /// To trigger a DiagnosticsStatusNotification request.
    DiagnosticsStatusNotification,
    /// To trigger a FirmwareStatusNotification request.
    FirmwareStatusNotification,
    /// To trigger a Heartbeat request.
    Heartbeat,
    /// To trigger a MeterValues request.
    MeterValues,
    /// To trigger a StatusNotification request.
    StatusNotification,
}

/// Phase as used in SampledValue. Phase specifies how a measured value
/// is to be interpreted. Please note that not all values of Phase are
/// applicable to all Measurands.
pub enum Phase {
    /// Measured on L1.
    L1,
    /// Measured on L2.
    L2,
    /// Measured on L3.
    L3,
    /// Measured on Neutral.
    N,
    /// Measured on L1 with respect to Neutral conductor.
    L1N,
    /// Measured on L2 with respect to Neutral conductor.
    L2N,
    /// Measured on L3 with respect to Neutral conductor.
    L3N,
    /// Measured between L1 and L2.
    L1L2,
    /// Measured between L2 and L3.
    L2L3,
    /// Measured between L3 and L1.
    L3L1,
}

/// Values of the context field of a value in SampledValue.
pub enum ReadingContext {
    /// Value taken at start of interruption.
    InterruptionBegin,
    /// Value taken when resuming after interruption.
    InterruptionEnd,
    /// Value for any other situations.
    Other,
    /// Value taken at clock aligned interval.
    SampleClock,
    /// Value taken as periodic sample relative to start time of transaction.
    SamplePeriodic,
    /// Value taken at end of transaction.
    TransactionBegin,
    /// Value taken at start of transaction.
    TransactionEnd,
    /// Value taken in response to a TriggerMessage.req.
    Trigger,
}

/// Reason for stopping a transaction in StopTransaction.req.
pub enum Reason {
    /// Emergency stop button was used.
    EmergencyStop,
    /// Disconnecting of cable, vehicle moved away from
    /// inductive charge unit.
    EVDisconnected,
    /// A hard reset command was received.
    HardReset,
    /// Stopped locally on request of the user at the
    /// Charge Point. This is reagular termination of a
    /// transaction. Examples: presenting an RFID tag,
    /// pressing a button to stop.
    Local,
    /// Any other reason.
    Other,
    /// Complete loss of power.
    PowerLoss,
    /// A locally initiated reset/reboot occurred (for
    /// instance watchdog kicked in).
    Reboot,
    /// Stopped remotely on request of the user. This is a
    /// regular termination of a transaction. Examples:
    /// termination using a smartphone app, exceeding a
    /// (non local) prepaid credit.
    Remote,
    /// A soft reset command was received.
    SoftReset,
    /// Central System sent an Unlock Connector command.
    UnlockCommand,
    /// The transaction was stopped because of the
    /// authorization status in a StartTransaction.conf.
    DeAuthorized,
}

pub enum RecurrencyKindType {
    /// The schedule restarts at the beginning of the next day.
    Daily,
    /// The schedule restarts at the beginning of the next week (defined as
    /// Monday morning).
    Weekly,
}

/// Result of registration in response to BootNotification.req.
pub enum RegistrationStatus {
    /// Charge point is accepted by Central System.
    Accepted,
    /// Central System is not yet ready to accept the
    /// Charge Point. Central System may send messages
    /// to retrieve information or prepare the Charge
    /// Point.
    Pending,
    /// Charge point is not accepted by Central System.
    /// This may happen when the Charge Point id is not
    /// known by Central System.
    Rejected,
}

/// The result of a RemoteStartTransaction.req or RemoteStopTransaction.req request.
pub enum RemoteStartStopStatus {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
}

/// Status in ReserveNow.conf.
pub enum ReservationStatus {
    /// Reservation has been made.
    Accepted,
    /// Reservation has not been made. All connectors or
    /// the specified connector are in a faulted state.
    Faulted,
    /// Reservation has not been made. All connector or
    /// the specified connector are occupied.
    Occupied,
    /// Reservation has not been made. Charge Point is
    /// not configured to accept reservations.
    Rejected,
    /// Reservation has not been made, because
    /// connectors or specified connector are in an
    /// unavailable state.
    Unavailable,
}

/// Result of Reset.req.
pub enum ResetStatus {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
}

/// Type of reset requested by Reset.req.
pub enum ResetType {
    /// Full reboot of Charge Point software.
    Hard,
    /// Return to initial status, gracefully terminating any
    /// transactions in progress.
    Soft,
}

/// Status in TriggerMessage.conf.
pub enum TriggerMessageStatus {
    /// Requested notification will be sent.
    Accepted,
    /// Requested notification will not be sent.
    Rejected,
    /// Requested notification cannot be sent because it is
    /// either not implemented or unknown.
    NotImplemented,
}

/// Allowable values of the optional "unit" field of a Value element, as used in
/// MeterValues.req and StopTransaction.req messages.
/// Default value of "unit" is always "Wh".
pub enum UnitOfMeasure {
    /// Watt-hours (energy). Default.
    WH,
    /// kiloWatt-hours (energy).
    KWH,
    /// Var-hours (reactive energy).
    VARH,
    /// kilovar-hours (reactive energy).
    KVARH,
    /// Watts (power).
    W,
    /// kilowatts (power).
    KW,
    /// VoltAmpere (apparent power).
    VA,
    /// kiloVoltAmpere (apparent power).
    KVA,
    /// Vars (reactive power).
    VAR,
    /// kilovars (reactive power).
    KVAR,
    /// Amperes (current).
    A,
    /// Voltage (r.m.s. AC).
    V,
    /// Degrees (temperature).
    Celsius,
    /// Degrees (temperature).
    Fahrenheit,
    /// Degrees Kelvin (temperature).
    K,
    /// Percentage.
    Percent,
}

/// Status in response to UnlockConnector.req.
pub enum UnlockStatus {
    /// Connector has successfully been unlocked.
    Unlocked,
    /// Failed to unlock the connector.
    UnlockFailed,
    /// Charge Point has no connector lock.
    NotSupported,
}

/// Type of update for a SendLocalList.req
pub enum UpdateStatus {
    /// Local Authorization List successfully updated.
    Accepted,
    /// Failed to update the Local Authorization List.
    Failed,
    /// Update of Local Authorization List is not
    /// supported by Charge Point.
    NotSupported,
    /// Version number in the request for a differential
    /// update is less or equal then version number of
    /// current list.
    VersionMismatch,
}

/// Type of update for a SendLocalList.req
pub enum UpdateType {
    /// Indicates that the current Local Authorization List
    /// must be updated with the values in this message.
    Differential,
    /// Indicates that the current Local Authorization List
    /// must be replaced by the values in this message.
    Full,
}

/// Format that specifies how the value element in SampledValue is to be interpreted.
pub enum ValueFormat {
    /// Data is to be interpreted as integer/decimal numeric data.
    Raw,
    /// Data is represented as a signed binary data block, encoded as hex data.
    SignedData,
}
