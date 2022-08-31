//! # Parameters
//!
//! This file contains all the values required to collect aphorisms and the courses

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub aphorisms: Vec<String>,
    pub courses: Vec<String>,
}

impl Parameters {
    /// Try to parse parameters from file path
    pub fn try_from_path(p: &Path) -> anyhow::Result<Self> {
        let mut file = BufReader::new(
            File::open(p)
                .map_err(|e| anyhow::anyhow!("could not open file {}: {}", p.display(), e))?,
        );
        serde_json::from_reader(&mut file)
            .map_err(|e| anyhow::anyhow!("failed to parse parameters: {}", e))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_parse_from_default_parameters() {
        assert!(Parameters::try_from_path(Path::new("config/parameters.json")).is_ok());
    }
}
