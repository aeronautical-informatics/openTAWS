use std::str::FromStr;

use cucumber::Parameter;
use lazy_static::lazy_static;
use opentaws::{Alert, AlertLevel};
use regex::Regex;
use uom::si::f64::Length;
use uom::si::length;

use super::constraints::Constraint;

pub struct MaybeParameter(bool);

impl Into<bool> for MaybeParameter {
    fn into(self) -> bool {
        self.0
    }
}

impl FromStr for MaybeParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maybe = s.trim().to_lowercase();
        maybe.retain(|c| !c.is_whitespace());
        match maybe.as_str() {
            "is" | "should" | "shall" => Ok(Self(true)),
            "isnot" | "shouldnot" | "shallnot" => Ok(Self(false)),
            _ => Err(format!("unknown word: {s}")),
        }
    }
}

impl Parameter for MaybeParameter {
    const NAME: &'static str = "maybe";
    const REGEX: &'static str = "(?:is|should|shall)\\s*(?:not)?";
}

pub struct AlertParameter(Alert);

impl FromStr for AlertParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut alert = s.trim().to_lowercase();
        alert.retain(|c| !c.is_whitespace());
        match alert.as_str() {
            "ffac" => Ok(Self(Alert::Ffac)),
            "flta" => Ok(Self(Alert::Flta)),
            "mode1" => Ok(Self(Alert::Mode1)),
            "mode2" => Ok(Self(Alert::Mode2)),
            "mode3" => Ok(Self(Alert::Mode3)),
            "mode4" => Ok(Self(Alert::Mode4)),
            "mode5" => Ok(Self(Alert::Mode5)),
            "pda" => Ok(Self(Alert::Pda)),
            _ => Err(format!("unknown alert: {s}")),
        }
    }
}

impl Into<Alert> for AlertParameter {
    fn into(self) -> Alert {
        self.0
    }
}

impl Parameter for AlertParameter {
    const NAME: &'static str = "alert";
    const REGEX: &'static str = "(?:[a-zA-Z]+\\s*[0-9]*)";
}

pub struct AlertLevelParameter(AlertLevel);

impl Into<AlertLevel> for AlertLevelParameter {
    fn into(self) -> AlertLevel {
        self.0
    }
}

impl FromStr for AlertLevelParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut alert_level = s.trim().to_lowercase();
        alert_level.retain(|c| !c.is_whitespace());
        match alert_level.as_str() {
            "warning" => Ok(Self(AlertLevel::Warning)),
            "caution" => Ok(Self(AlertLevel::Caution)),
            "annunciation" => Ok(Self(AlertLevel::Annunciation)),
            _ => Err(format!("invalid alert level: {}", s)),
        }
    }
}

impl Parameter for AlertLevelParameter {
    const NAME: &'static str = "alert_level";
    const REGEX: &'static str = "(?:[Ww]arning|[Cc]aution|[Aa]nnunciation)";
}

pub struct ConstraintParameter(Constraint<f64>);

impl FromStr for ConstraintParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PATTERN: &str = concat!(
            "(?P<type>at least|at most|within|between|not between)",
            "\\s*(?P<q1>[+-]?(?:[0-9]*[.])?[0-9]+)",
            "(?:\\s*and\\s*(?P<q2>[+-]?(?:[0-9]*[.])?[0-9]+))?"
        );

        lazy_static! {
            static ref REGEX: Regex = Regex::new(PATTERN).unwrap();
        }

        let captures = REGEX.captures(s).ok_or("invalid constraint string")?;
        let typ = captures
            .name("type")
            .ok_or("constaint type not found")?
            .as_str();
        let q1 = captures.name("q1").ok_or("quantity not found")?.as_str();
        let q2 = captures.name("q2").map(|x| x.as_str());

        let q1 = q1.parse::<f64>().map_err(|_| "invalid quantity format")?;
        let q2 = q2
            .map(|x| x.parse::<f64>())
            .transpose()
            .map_err(|_| "invalid quantity format")?;

        match typ {
            "at least" => match q2 {
                Some(_) => Err(format!("unexpected: {}", s)),
                None => Ok(Self(Constraint::AtLeast(0, q1))),
            },
            "at most" | "within" => match q2 {
                Some(_) => Err(format!("unexpected: {}", s)),
                None => Ok(Self(Constraint::AtMost(0, q1))),
            },
            "between" => match q2 {
                Some(q2) if q1 <= q2 => Ok(Self(Constraint::InRange(0, q1, q2))),
                Some(q2) => Ok(Self(Constraint::InRange(0, q2, q1))),
                None => Err(format!("missing second bound: {}", s)),
            },
            "not between" => match q2 {
                Some(q2) if q1 <= q2 => Ok(Self(Constraint::NotInRange(0, q1, q2))),
                Some(q2) => Ok(Self(Constraint::NotInRange(0, q2, q1))),
                None => Err(format!("missing second bound: {}", s)),
            },
            _ => Err(format!("invalid constraint type: {}", s)),
        }
    }
}

impl Into<Constraint<f64>> for ConstraintParameter {
    fn into(self) -> Constraint<f64> {
        self.0
    }
}

impl Parameter for ConstraintParameter {
    const NAME: &'static str = "constraint";

    const REGEX: &'static str = concat!(
        "(?:at least|at most|within|between|not between)",
        "\\s*[+-]?(?:[0-9]*[.])?[0-9]+",
        "(?:\\s*and\\s*[+-]?(?:[0-9]*[.])?[0-9]+)?"
    );
}
