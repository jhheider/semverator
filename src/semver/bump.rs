use anyhow::{bail, Result};

use super::Semver;

impl Semver {
    pub fn bump(&self, which: &SemverComponent) -> Result<Self> {
        match which {
            SemverComponent::Major => Self::from((self.major + 1, 0, 0)),
            SemverComponent::Minor => Self::from((self.major, self.minor + 1, 0)),
            SemverComponent::Patch => Self::from((self.major, self.minor, self.patch + 1)),
            SemverComponent::None => Ok(self.clone()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemverComponent {
    Major,
    Minor,
    Patch,
    None,
}

impl SemverComponent {
    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "patch" => Ok(Self::Patch),
            _ => bail!("invalid bump component '{}'", s),
        }
    }
}

impl From<&str> for SemverComponent {
    fn from(s: &str) -> Self {
        Self::parse(s).unwrap_or(Self::None)
    }
}
