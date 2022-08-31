//! # Aphorism
//!
//! This module contains the papi's aphorisms

use rand::seq::SliceRandom;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AphorismJar {
    aphorisms: Vec<String>,
    index: AtomicUsize,
}

impl AphorismJar {
    /// Get a random aphorism of the papi
    pub fn get_random(&self) -> &str {
        if self.index.load(Ordering::Relaxed) >= self.aphorisms.len() {
            self.index.store(0, Ordering::SeqCst);
        }
        let aphorism = self
            .aphorisms
            .get(self.index.load(Ordering::Relaxed))
            .map(|x| x.as_str())
            .unwrap_or_default();
        self.index.fetch_add(1, Ordering::SeqCst);
        aphorism
    }
}

impl From<&[String]> for AphorismJar {
    fn from(aphorisms: &[String]) -> Self {
        let mut aphorisms: Vec<String> = aphorisms.iter().map(|x| x.to_string()).collect();
        let mut rng = rand::thread_rng();
        aphorisms.shuffle(&mut rng);
        Self {
            aphorisms,
            index: AtomicUsize::new(0),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::big_luca::Parameters;

    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn should_get_random_aphorism() {
        let parameters = Parameters::try_from_path(Path::new("config/parameters.json")).unwrap();
        let aphorism = AphorismJar::from(parameters.aphorisms.as_slice());
        assert!(!aphorism.get_random().is_empty());
        assert!(parameters
            .aphorisms
            .contains(&aphorism.get_random().to_string()));
    }

    #[test]
    fn should_wrap_and_increment_index() {
        let parameters = Parameters::try_from_path(Path::new("config/parameters.json")).unwrap();
        let aphorism = AphorismJar::from(&parameters.aphorisms[0..2]);
        assert_eq!(aphorism.aphorisms.len(), 2);
        assert_eq!(aphorism.aphorisms[0], aphorism.get_random());
        assert_eq!(aphorism.aphorisms[1], aphorism.get_random());
        assert_eq!(aphorism.aphorisms[0], aphorism.get_random());
    }
}
