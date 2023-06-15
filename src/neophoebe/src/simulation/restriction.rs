use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Restriction {
    NoRestriction,
    CommunityRestriction(usize, f64, usize),
    LowerCutOffRestriction(usize, f64, usize),
    UpperCutOffRestriction(usize, f64, usize),
    PersonalRestriction(f64),
}