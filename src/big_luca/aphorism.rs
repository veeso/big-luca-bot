//! # Aphorism
//!
//! This module contains the papi's aphorisms

use super::PARAMETERS;

use rand::Rng;

pub struct Aphorism;

impl Aphorism {
    /// Get a random aphorism of the papi
    pub fn get_random() -> String {
        let mut rng = rand::thread_rng();
        let aphorisms = PARAMETERS.get().unwrap().aphorisms.as_slice();
        aphorisms[rng.gen_range(0..aphorisms.len())].clone()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::big_luca::Parameters;
    use std::path::Path;

    #[test]
    fn should_get_random_aphorism() {
        assert!(PARAMETERS
            .set(Parameters::try_from_path(Path::new("config/parameters.json")).unwrap())
            .is_ok());
        assert!(!Aphorism::get_random().is_empty());
        assert!(PARAMETERS
            .get()
            .unwrap()
            .aphorisms
            .contains(&Aphorism::get_random()))
    }
}
