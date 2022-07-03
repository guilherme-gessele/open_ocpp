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
