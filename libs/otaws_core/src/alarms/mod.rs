/// The different types of possible reports
#[derive(Debug, PartialEq)]
pub enum Report {
    /// Forward Lookig Terrain Avoidance
    FLTA,

    /// Premature Descent Alerting
    PDA,

    /// Excessive Rate of Descent
    Mode1,

    /// Excessive ClosureRate to Terrain
    Mode2,

    /// Negative Climb Rate or Altitude Loss after Take-off or Go Around
    Mode3,

    /// Flight Near Terrain when Not in Landing Configuration
    Mode4,

    /// Excessive Downward Deviation from an ILS Glideslope or LPV/GLS Glidepath
    Mode5,

    /// Five Hundred Foot Altitude Callout
    FFAC,
    // TODO add more
}
