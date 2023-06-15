use serde::{Deserialize, Serialize};

/// An enum for all possible restriction plans in the simulation
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Restriction {
    NoRestriction,
    CommunityRestriction(usize, f64, usize),
    LowerCutOffRestriction(usize, f64, usize),
    UpperCutOffRestriction(usize, f64, usize),
    PersonalRestriction(f64),
}