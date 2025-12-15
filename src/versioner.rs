use crate::analyser::BumpType;
use crate::error::{BumperError, BumperResult};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn parse(version_str: &str) -> BumperResult<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();

        if parts.len() != 3 {
            return Err(BumperError::InvalidVersion(version_str.to_string()));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| BumperError::InvalidVersion(version_str.to_string()))?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| BumperError::InvalidVersion(version_str.to_string()))?;
        let patch = parts[2]
            .parse::<u32>()
            .map_err(|_| BumperError::InvalidVersion(version_str.to_string()))?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }

    pub fn bump(&self, bump_type: BumpType) -> Self {
        match bump_type {
            BumpType::Major => Version {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
            BumpType::Minor => Version {
                major: self.major,
                minor: self.minor + 1,
                patch: 0,
            },
            BumpType::Patch => Version {
                major: self.major,
                minor: self.minor,
                patch: self.patch + 1,
            },
            BumpType::None => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        let version = Version::parse("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_bump_major() {
        let version = Version::parse("1.2.3").unwrap();
        let bumped = version.bump(BumpType::Major);
        assert_eq!(bumped.to_string(), "2.0.0");
    }

    #[test]
    fn test_version_bump_minor() {
        let version = Version::parse("1.2.3").unwrap();
        let bumped = version.bump(BumpType::Minor);
        assert_eq!(bumped.to_string(), "1.3.0");
    }

    #[test]
    fn test_version_bump_patch() {
        let version = Version::parse("1.2.3").unwrap();
        let bumped = version.bump(BumpType::Patch);
        assert_eq!(bumped.to_string(), "1.2.4");
    }
}
