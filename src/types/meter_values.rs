use chrono::{DateTime, Utc};

use super::enumerations::{Location, Measurand, Phase, ReadingContext, UnitOfMeasure, ValueFormat};

pub struct MeterValues {
    /// Timestamp for measured value(s).
    timestamp: DateTime<Utc>,
    /// One or more measured values.
    sampled_value: Vec<SampledValue>,
}

pub struct SampledValue {
    /// Value as a "Raw" (decimal) number or "SigedData".
    /// Field Type is "string" to allow for digitally signed data readings.
    /// Decimal numeric values are so acceptable to allow fractional
    /// values for measurands such as Temperature and Current.
    value: String,
    /// Type of detail value: start, end or sample.
    /// Default = "Sample.Periodic".
    context: Option<ReadingContext>,
    /// Raw or signed data.
    /// Default = "Raw".
    format: Option<ValueFormat>,
    /// Type of measurement.
    /// Default = "Energy.Active.Import.Register".
    measurand: Option<Measurand>,
    /// Indicates how the measured value is to be interpreted.
    /// For instance between L1 and neutral (L1-N).
    /// Please note that not all values of phase are applicable
    /// to all Measurands.
    /// When phase is absent, the measured value is interpreted
    /// as an overall value.
    phase: Option<Phase>,
    /// Location of measurement.
    /// Default = "Outlet".
    location: Option<Location>,
    /// Unit of the value.
    /// Default = "Wh" if the (default) measurand is an "Energy" type.
    unit: Option<UnitOfMeasure>,
}
