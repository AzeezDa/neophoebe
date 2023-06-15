use serde::{Serialize, Deserialize};
use std::fs;

use super::Restriction;

/// A struct that stores all the simulation's parameters (parsed using `serde`)
#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub population_size: usize,
    pub household_size: usize,
    pub household_relation: f64,
    pub extra_relations: Vec<(usize, f64, usize)>, // #RelationGroupSize, RelationStrength, #Applications
    pub hygiencity: f64,
    pub disease_spread: f64,
    pub disease_incubation: f64,
    pub disease_recovery: f64,
    pub disease_mortality: f64,
    pub tests_per_day: usize,
    pub restriction_plan: Restriction
}

impl Parameters {
    /// Sets up the parameters using the given filepath to the parameters RON file
    pub fn read(file_path: String) -> Result<Self, &'static str> {
        if let Ok(s) = fs::read_to_string(file_path) {
            if let Ok(params) = ron::from_str(&s) {
                return Ok(params);
            }
        }

        Err("Error reading parameters file")
    }
}